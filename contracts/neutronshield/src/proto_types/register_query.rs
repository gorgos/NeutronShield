// This file is generated by rust-protobuf 2.28.0. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]


//! Generated file from `neutron/proto/interchainqueries/tx.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_28_0;

#[derive(PartialEq, Clone, Default, Debug)]
pub struct MsgRegisterInterchainQueryResponse {
    // message fields
    pub id: u64,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a MsgRegisterInterchainQueryResponse {
    fn default() -> &'a MsgRegisterInterchainQueryResponse {
        <MsgRegisterInterchainQueryResponse as ::protobuf::Message>::default_instance()
    }
}

impl MsgRegisterInterchainQueryResponse {
    pub fn new() -> MsgRegisterInterchainQueryResponse {
        ::std::default::Default::default()
    }

    // uint64 id = 1;

    pub fn get_id(&self) -> u64 {
        self.id
    }
    pub fn clear_id(&mut self) {
        self.id = 0;
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u64) {
        self.id = v;
    }
}

impl ::protobuf::Message for MsgRegisterInterchainQueryResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(
        &mut self,
        is: &mut ::protobuf::CodedInputStream<'_>,
    ) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(
                            wire_type,
                        ));
                    }
                    let tmp = is.read_uint64()?;
                    self.id = tmp;
                }
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(
                        field_number,
                        wire_type,
                        is,
                        self.mut_unknown_fields(),
                    )?;
                }
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.id != 0 {
            my_size +=
                ::protobuf::rt::value_size(1, self.id, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(
        &self,
        os: &mut ::protobuf::CodedOutputStream<'_>,
    ) -> ::protobuf::ProtobufResult<()> {
        if self.id != 0 {
            os.write_uint64(1, self.id)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> MsgRegisterInterchainQueryResponse {
        MsgRegisterInterchainQueryResponse::new()
    }

    fn default_instance() -> &'static MsgRegisterInterchainQueryResponse {
        static instance: ::protobuf::rt::LazyV2<MsgRegisterInterchainQueryResponse> =
            ::protobuf::rt::LazyV2::INIT;
        instance.get(MsgRegisterInterchainQueryResponse::new)
    }
}

impl ::protobuf::Clear for MsgRegisterInterchainQueryResponse {
    fn clear(&mut self) {
        self.id = 0;
        self.unknown_fields.clear();
    }
}

impl ::protobuf::reflect::ProtobufValue for MsgRegisterInterchainQueryResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}
