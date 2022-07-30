// The following code was taken from https://github.com/feather-rs/feather/blob/main/feather/protocol/src/packets.rs which is licensed under Apache 2.0.
// It was modified slightly to fit the needs of this crate.

#[macro_export]
macro_rules! user_type {
    (VarInt) => {
        i32
    };
    (VarIntPrefixedArray <$inner:ident>) => {
        Vec<$inner>
    };
    (ShortPrefixedArray <$inner:ident>) => {
        Vec<$inner>
    };
    (LengthInferredByteArray) => {
        Vec<u8>
    };
    (Angle) => {
        f32
    };
    ($typ:ty) => {
        $typ
    };
}

#[macro_export]
macro_rules! user_type_convert_to_writeable {
    (VarInt, $e:expr) => {
        VarInt(*$e as i32)
    };
    (VarIntPrefixedArray <$inner:ident>, $e:expr) => {
        VarIntPrefixedArray::from($e.as_slice())
    };
    (ShortPrefixedArray <$inner:ident>, $e:expr) => {
        ShortPrefixedArray::from($e.as_slice())
    };
    (LengthInferredByteArray, $e:expr) => {
        LengthInferredByteArray::from($e.as_slice())
    };
    (Angle, $e:expr) => {
        Angle(*$e)
    };
    ($typ:ty, $e:expr) => {
        $e
    };
}

#[macro_export]
macro_rules! packets {
    (
        $(
            $packet:ident($id:expr) {
                $(
                    $field:ident $typ:ident $(<$generics:ident>)?
                );* $(;)?
            } $(,)?
        )*
    ) => {
        $(
            #[allow(unused_imports)]
            use $crate::net::types::*;
            #[allow(unused_imports)]
            use $crate::net::packet::*;
            #[allow(unused_imports)]
            use $crate::*;

            #[derive(Debug, Clone)]
            pub struct $packet {
                $(
                    pub $field: user_type!($typ $(<$generics>)?),
                )*
            }

            impl Packet for $packet {
                const ID: VarInt = VarInt($id);
            }

            #[allow(unused_imports, unused_variables)]
            impl Decoder for $packet {
                fn read_from(buffer: &mut impl std::io::Read) -> anyhow::Result<Self>
                where
                    Self: Sized
                {
                    use anyhow::Context as _;
                    $(
                        let $field = <$typ $(<$generics>)?>::read_from(buffer)
                            .context(concat!("failed to read field `", stringify!($field), "` of packet `", stringify!($packet), "`"))?
                            .into();
                    )*

                    Ok(Self {
                        $(
                            $field,
                        )*
                    })
                }
            }

            #[allow(unused_variables)]
            impl Encoder for $packet {
                fn write_to(&self, w: &mut impl std::io::Write) -> anyhow::Result<()> {
                    $(
                        user_type_convert_to_writeable!($typ $(<$generics>)?, &self.$field).write_to(w)?;
                    )*
                    Ok(())
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! discriminant_to_literal {
    (String, $discriminant:expr) => {
        &*$discriminant
    };
    ($discriminant_type:ident, $discriminant:expr) => {
        $discriminant.into()
    };
}

#[macro_export]
macro_rules! def_enum {
    (
        $ident:ident ($discriminant_type:ident) {
            $(
                $discriminant:literal = $variant:ident
                $(
                    {
                        $(
                            $field:ident $typ:ident $(<$generics:ident>)?
                        );* $(;)?
                    }
                )?
            ),* $(,)?
        }
    ) => {
        #[derive(Debug, Clone)]
        pub enum $ident {
            $(
                $variant
                $(
                    {
                        $(
                            $field: user_type!($typ $(<$generics>)?),
                        )*
                    }
                )?,
            )*
        }

        impl $crate::net::types::Decoder for $ident {
            fn read_from(buffer: &mut impl std::io::Read) -> anyhow::Result<Self>
                where
                    Self: Sized
            {
                use anyhow::Context as _;
                let discriminant = <$discriminant_type>::read_from(buffer)
                    .context(concat!("failed to read discriminant for enum type ", stringify!($ident)))?;

                match discriminant_to_literal!($discriminant_type, discriminant) {
                    $(
                        $discriminant => {
                            $(
                                $(
                                    let $field = <$typ $(<$generics>)?>::read(buffer, version)
                                        .context(concat!("failed to read field `", stringify!($field),
                                            "` of enum `", stringify!($ident), "::", stringify!($variant), "`"))?
                                            .into();
                                )*
                            )?

                            Ok($ident::$variant $(
                                {
                                    $(
                                        $field,
                                    )*
                                }
                            )?)
                        },
                    )*
                    _ => Err(anyhow::anyhow!(
                        concat!(
                            "no discriminant for enum `", stringify!($ident), "` matched value {:?}"
                        ), discriminant
                    ))
                }
            }
        }

        impl $crate::net::types::Encoder for $ident {
            fn write_to(&self, buffer: &mut impl std::io::Write) -> anyhow::Result<()> {
                match self {
                    $(
                        $ident::$variant $(
                            {
                                $($field,)*
                            }
                        )? => {
                            let discriminant = <$discriminant_type>::from($discriminant);
                            discriminant.write_to(buffer)?;

                            $(
                                $(
                                    user_type_convert_to_writeable!($typ $(<$generics>)?, $field).write(buffer, version)?;
                                )*
                            )?
                        }
                    )*
                }
                Ok(())
            }
        }
    };
}

/*
#[macro_export]
macro_rules! packet_enum {
    (
        $ident:ident {
            $($id:literal = $packet:ident),* $(,)?
        }
    ) => {
        #[derive(Debug, Clone)]
        pub enum $ident {
            $(
                $packet($packet),
            )*
        }

        impl $ident {
            /// Returns the packet ID of this packet.
            pub fn id(&self) -> u32 {
                match self {
                    $(
                        $ident::$packet(_) => $id,
                    )*
                }
            }
        }

        impl crate::Readable for $ident {
            fn read_from(buffer: &mut ::std::io::Cursor<&[u8]>, version: crate::ProtocolVersion) -> anyhow::Result<Self>
            where
                Self: Sized
            {
                let packet_id = VarInt::read(buffer, version)?.0;
                match packet_id {
                    $(
                        id if id == $id => Ok($ident::$packet($packet::read(buffer, version)?)),
                    )*
                    _ => Err(anyhow::anyhow!("unknown packet ID {}", packet_id)),
                }
            }
        }

        impl crate::Writeable for $ident {
            fn write_to(&self, buffer: &mut Vec<u8>, version: crate::ProtocolVersion) -> anyhow::Result<()> {
                VarInt(self.id() as i32).write(buffer, version)?;
                match self {
                    $(
                        $ident::$packet(packet) => {
                            packet.write(buffer, version)?;
                        }
                    )*
                }
                Ok(())
            }
        }

        $(
            impl VariantOf<$ident> for $packet {
                fn discriminant_id() -> u32 { $id }

                #[allow(unreachable_patterns)]
                fn destructure(e: $ident) -> Option<Self> {
                    match e {
                        $ident::$packet(p) => Some(p),
                        _ => None,
                    }
                }
            }

            impl From<$packet> for $ident {
                fn from(packet: $packet) -> Self {
                    $ident::$packet(packet)
                }
            }
        )*
    }
}
*/
