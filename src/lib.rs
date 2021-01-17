#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
pub mod google {
    pub mod datastore {
        pub mod v1 {
            tonic::include_proto!("google.datastore.v1");
        }
        pub mod v1beta3 {
            tonic::include_proto!("google.datastore.v1beta3");
        }
    }
    //pub mod rpc {
    //    tonic::include_proto!("google.rpc");
    //}
    pub mod r#type {
        tonic::include_proto!("google.r#type");
    }
}

pub use google::datastore::*;
pub use tonic;
