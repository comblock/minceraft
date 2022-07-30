// very much WIP
// minceraft-derive is unused for now as a packet builder macro is easier to use
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, spanned::Spanned, LitInt, AttrStyle};

#[proc_macro_derive(Packet, attributes(id))]
pub fn derive_packet(input: TokenStream) -> TokenStream {
	let ast: syn::DeriveInput = parse_macro_input!(input);
	let name = ast.ident;

	let attrs = ast.attrs;

	let mut id = Option::<LitInt>::None;

	for i in attrs {
		if let AttrStyle::Outer = i.style {
			id = Some(i.parse_args().unwrap());
		}
	}
	
	let id = id.unwrap();
	

	let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(ref fields),
        ..
    }) = ast.data
    {
        fields
    } else {
        panic!("Only support structs")
    };
	
	let recurse_encoder = fields.named.iter().map(|f| {
		let name = &f.ident;
		quote_spanned! {f.span()=>
			self.#name.write_to(w)?;
		}
	});
	let stream_encoder = quote! {
		#(#recurse_encoder)*
	};

	let recurse_decoder_types = fields.named.iter().map(|f| {
		let ty = &f.ty;
		let name = &f.ident;
		quote_spanned! {f.span()=>
			let #name = <#ty>::read_from(r)?;
		}
	});
	let stream_decoder_types = quote! {
		#(#recurse_decoder_types)*
	};

	let recurse_decoder_names = fields.named.iter().map(|f| {
		let name = &f.ident;
		quote_spanned! {f.span()=>
			#name,
		}
	});
	let stream_decoder_names = quote! {
		#(#recurse_decoder_names)*
	};

	let extended = quote! {
		impl minceraft::net::types::Encoder for #name {
			fn write_to(&self, w: &mut impl std::io::Write) -> anyhow::Result<()> {
				use minceraft::net::types::Encoder;
				#stream_encoder
				Ok(())
			}
		}
		
		impl minceraft::net::types::Decoder for #name {
			fn read_from(r: &mut impl std::io::Read) -> anyhow::Result<Self> {
				use minceraft::net::types::Decoder;
				#stream_decoder_types

				Ok(Self {
					#stream_decoder_names
				})
			}
		}

		impl minceraft::net::packet::Packet for #name {
			const ID: minceraft::net::types::VarInt = minceraft::net::types::VarInt(#id);
		}
	};
	TokenStream::from(extended)
}
