// This file is generated by rust-protobuf 3.4.0. Do not edit
// .proto file is parsed by protoc 25.2
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `authority_keys.proto`
// Generated for lite runtime

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:cast.channel.AuthorityKeys)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct AuthorityKeys {
    // message fields
    // @@protoc_insertion_point(field:cast.channel.AuthorityKeys.keys)
    pub keys: ::std::vec::Vec<authority_keys::Key>,
    // special fields
    // @@protoc_insertion_point(special_field:cast.channel.AuthorityKeys.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AuthorityKeys {
    fn default() -> &'a AuthorityKeys {
        <AuthorityKeys as ::protobuf::Message>::default_instance()
    }
}

impl AuthorityKeys {
    pub fn new() -> AuthorityKeys {
        ::std::default::Default::default()
    }
}

impl ::protobuf::Message for AuthorityKeys {
    const NAME: &'static str = "AuthorityKeys";

    fn is_initialized(&self) -> bool {
        for v in &self.keys {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.keys.push(is.read_message()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        for value in &self.keys {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.keys {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> AuthorityKeys {
        AuthorityKeys::new()
    }

    fn clear(&mut self) {
        self.keys.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AuthorityKeys {
        static instance: AuthorityKeys = AuthorityKeys {
            keys: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

/// Nested message and enums of message `AuthorityKeys`
pub mod authority_keys {
    // @@protoc_insertion_point(message:cast.channel.AuthorityKeys.Key)
    #[derive(PartialEq,Clone,Default,Debug)]
    pub struct Key {
        // message fields
        // @@protoc_insertion_point(field:cast.channel.AuthorityKeys.Key.fingerprint)
        pub fingerprint: ::std::option::Option<::std::vec::Vec<u8>>,
        // @@protoc_insertion_point(field:cast.channel.AuthorityKeys.Key.public_key)
        pub public_key: ::std::option::Option<::std::vec::Vec<u8>>,
        // special fields
        // @@protoc_insertion_point(special_field:cast.channel.AuthorityKeys.Key.special_fields)
        pub special_fields: ::protobuf::SpecialFields,
    }

    impl<'a> ::std::default::Default for &'a Key {
        fn default() -> &'a Key {
            <Key as ::protobuf::Message>::default_instance()
        }
    }

    impl Key {
        pub fn new() -> Key {
            ::std::default::Default::default()
        }

        // required bytes fingerprint = 1;

        pub fn fingerprint(&self) -> &[u8] {
            match self.fingerprint.as_ref() {
                Some(v) => v,
                None => &[],
            }
        }

        pub fn clear_fingerprint(&mut self) {
            self.fingerprint = ::std::option::Option::None;
        }

        pub fn has_fingerprint(&self) -> bool {
            self.fingerprint.is_some()
        }

        // Param is passed by value, moved
        pub fn set_fingerprint(&mut self, v: ::std::vec::Vec<u8>) {
            self.fingerprint = ::std::option::Option::Some(v);
        }

        // Mutable pointer to the field.
        // If field is not initialized, it is initialized with default value first.
        pub fn mut_fingerprint(&mut self) -> &mut ::std::vec::Vec<u8> {
            if self.fingerprint.is_none() {
                self.fingerprint = ::std::option::Option::Some(::std::vec::Vec::new());
            }
            self.fingerprint.as_mut().unwrap()
        }

        // Take field
        pub fn take_fingerprint(&mut self) -> ::std::vec::Vec<u8> {
            self.fingerprint.take().unwrap_or_else(|| ::std::vec::Vec::new())
        }

        // required bytes public_key = 2;

        pub fn public_key(&self) -> &[u8] {
            match self.public_key.as_ref() {
                Some(v) => v,
                None => &[],
            }
        }

        pub fn clear_public_key(&mut self) {
            self.public_key = ::std::option::Option::None;
        }

        pub fn has_public_key(&self) -> bool {
            self.public_key.is_some()
        }

        // Param is passed by value, moved
        pub fn set_public_key(&mut self, v: ::std::vec::Vec<u8>) {
            self.public_key = ::std::option::Option::Some(v);
        }

        // Mutable pointer to the field.
        // If field is not initialized, it is initialized with default value first.
        pub fn mut_public_key(&mut self) -> &mut ::std::vec::Vec<u8> {
            if self.public_key.is_none() {
                self.public_key = ::std::option::Option::Some(::std::vec::Vec::new());
            }
            self.public_key.as_mut().unwrap()
        }

        // Take field
        pub fn take_public_key(&mut self) -> ::std::vec::Vec<u8> {
            self.public_key.take().unwrap_or_else(|| ::std::vec::Vec::new())
        }
    }

    impl ::protobuf::Message for Key {
        const NAME: &'static str = "Key";

        fn is_initialized(&self) -> bool {
            if self.fingerprint.is_none() {
                return false;
            }
            if self.public_key.is_none() {
                return false;
            }
            true
        }

        fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
            while let Some(tag) = is.read_raw_tag_or_eof()? {
                match tag {
                    10 => {
                        self.fingerprint = ::std::option::Option::Some(is.read_bytes()?);
                    },
                    18 => {
                        self.public_key = ::std::option::Option::Some(is.read_bytes()?);
                    },
                    tag => {
                        ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                    },
                };
            }
            ::std::result::Result::Ok(())
        }

        // Compute sizes of nested messages
        #[allow(unused_variables)]
        fn compute_size(&self) -> u64 {
            let mut my_size = 0;
            if let Some(v) = self.fingerprint.as_ref() {
                my_size += ::protobuf::rt::bytes_size(1, &v);
            }
            if let Some(v) = self.public_key.as_ref() {
                my_size += ::protobuf::rt::bytes_size(2, &v);
            }
            my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
            self.special_fields.cached_size().set(my_size as u32);
            my_size
        }

        fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
            if let Some(v) = self.fingerprint.as_ref() {
                os.write_bytes(1, v)?;
            }
            if let Some(v) = self.public_key.as_ref() {
                os.write_bytes(2, v)?;
            }
            os.write_unknown_fields(self.special_fields.unknown_fields())?;
            ::std::result::Result::Ok(())
        }

        fn special_fields(&self) -> &::protobuf::SpecialFields {
            &self.special_fields
        }

        fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
            &mut self.special_fields
        }

        fn new() -> Key {
            Key::new()
        }

        fn clear(&mut self) {
            self.fingerprint = ::std::option::Option::None;
            self.public_key = ::std::option::Option::None;
            self.special_fields.clear();
        }

        fn default_instance() -> &'static Key {
            static instance: Key = Key {
                fingerprint: ::std::option::Option::None,
                public_key: ::std::option::Option::None,
                special_fields: ::protobuf::SpecialFields::new(),
            };
            &instance
        }
    }
}
