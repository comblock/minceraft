![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/comblock/minceraft?style=for-the-badge)
![GitHub last commit](https://img.shields.io/github/last-commit/comblock/minceraft?style=for-the-badge)

# minceraft
Minceraft is a library for minecraft related stuff.
Currently it is divided into 2 modules that have to be enabled as a feature:
- net
- auth


## net
The net module provides an API for minecraft networking. It's primarily focussed at version 1.8.9 and clients, but you should be able to use it for servers and for other versions as well.

## Auth
The auth module is for logging into a minecraft account by using the microsoft oauth2 device flow: https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-device-code
It also caches the token with a custom binary format that is base64 encoded so you can easily copy paste it (although you should almost never need to do this!).
### Example
```rs
use {ms_auth_mc::*, reqwest::blocking::Client};

let client = Client::new();
let device_code =
    DeviceCode::new("389b1b32-b5d5-43b2-bddc-84ce938d6737"/* You would ideally replace this with your own CID*/, None, &client).unwrap();
 
if let Some(inner) = &device_code.inner {
   println!("{}", inner.message);
}
 
let auth = device_code.authenticate(&client).unwrap();
println!("{}", auth.token);
```
You can create your own cid by making an azure application.
