// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]


// interface

pub trait Messages {
    fn put(&self, o: ::grpc::RequestOptions, p: super::datasquid::PutRequest) -> ::grpc::SingleResponse<super::datasquid::PutResponse>;

    fn list_collections(&self, o: ::grpc::RequestOptions, p: super::datasquid::ListCollectionsRequest) -> ::grpc::SingleResponse<super::datasquid::ListCollectionsResponse>;

    fn create_collection(&self, o: ::grpc::RequestOptions, p: super::datasquid::CreateCollectionRequest) -> ::grpc::SingleResponse<super::datasquid::CreateCollectionResponse>;
}

// client

pub struct MessagesClient {
    grpc_client: ::grpc::Client,
    method_Put: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::datasquid::PutRequest, super::datasquid::PutResponse>>,
    method_ListCollections: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::datasquid::ListCollectionsRequest, super::datasquid::ListCollectionsResponse>>,
    method_CreateCollection: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::datasquid::CreateCollectionRequest, super::datasquid::CreateCollectionResponse>>,
}

impl MessagesClient {
    pub fn with_client(grpc_client: ::grpc::Client) -> Self {
        MessagesClient {
            grpc_client: grpc_client,
            method_Put: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/org.datasquid.protocol.Messages/Put".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ListCollections: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/org.datasquid.protocol.Messages/ListCollections".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_CreateCollection: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/org.datasquid.protocol.Messages/CreateCollection".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }

    pub fn new_plain(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_plain(host, port, conf).map(|c| {
            MessagesClient::with_client(c)
        })
    }
    pub fn new_tls<C : ::tls_api::TlsConnector>(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_tls::<C>(host, port, conf).map(|c| {
            MessagesClient::with_client(c)
        })
    }
}

impl Messages for MessagesClient {
    fn put(&self, o: ::grpc::RequestOptions, p: super::datasquid::PutRequest) -> ::grpc::SingleResponse<super::datasquid::PutResponse> {
        self.grpc_client.call_unary(o, p, self.method_Put.clone())
    }

    fn list_collections(&self, o: ::grpc::RequestOptions, p: super::datasquid::ListCollectionsRequest) -> ::grpc::SingleResponse<super::datasquid::ListCollectionsResponse> {
        self.grpc_client.call_unary(o, p, self.method_ListCollections.clone())
    }

    fn create_collection(&self, o: ::grpc::RequestOptions, p: super::datasquid::CreateCollectionRequest) -> ::grpc::SingleResponse<super::datasquid::CreateCollectionResponse> {
        self.grpc_client.call_unary(o, p, self.method_CreateCollection.clone())
    }
}

// server

pub struct MessagesServer;


impl MessagesServer {
    pub fn new_service_def<H : Messages + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/org.datasquid.protocol.Messages",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/org.datasquid.protocol.Messages/Put".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.put(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/org.datasquid.protocol.Messages/ListCollections".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.list_collections(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/org.datasquid.protocol.Messages/CreateCollection".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.create_collection(o, p))
                    },
                ),
            ],
        )
    }
}
