use std::future::ready;
use std::future::Ready;
use actix_identity::error::GetIdentityError;
use actix_identity::Identity;
use actix_web::{ FromRequest, Error, HttpResponse };

// This struct is used as a wrapper around actix_identity::Identity
// It allows implementation of Apiv2Schema and OperationModifier traits which are needed by paperclip documentation
// It also allows greater flexibility when dealing with errors
pub struct UserIdentity(pub actix_identity::Identity);

impl UserIdentity {
    pub fn id(&self) -> Result<String, GetIdentityError> {
        self.0.id()
    }
}

impl FromRequest for UserIdentity {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let identity = Identity::from_request(req, _payload).into_inner();

        match identity {
            Ok(identity) => ready(Ok(UserIdentity(identity))),
            Err(e) => {
                let res = actix_web::error::InternalError::from_response(
                    e,
                    HttpResponse::Unauthorized().body("Unauthorized")
                );
    
                ready(Err(actix_web::Error::from(res)))
            }
        }
    }
}