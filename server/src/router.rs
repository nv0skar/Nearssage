// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

#[async_trait]
pub trait Router<T: Sized> {
    async fn route(self) -> Result<Option<T>>;
}

#[async_trait]
impl Router<ServerCodec> for ClientCodec {
    #[instrument(skip_all, err)]
    async fn route(self) -> Result<Option<ServerCodec>> {
        // match self {
        //     ClientCodec::Auth(auth) => match auth {
        //         Auth::Login {
        //             pk_keychain,
        //             device,
        //             password,
        //         } => todo!(),
        //         Auth::Register {
        //             pk_keychain,
        //             device,
        //             username,
        //             profile,
        //             contact,
        //             password,
        //         } => todo!(),
        //         Auth::Challenge => todo!(),
        //         Auth::Elevate { device, challenge } => todo!(),
        //     },
        //     ClientCodec::Session(session) => match session {
        //         Session::OneTime(_) => todo!(),
        //         Session::Invalidate => todo!(),
        //     },
        //     ClientCodec::User(user) => match user {
        //         UserReq::Username(_) => todo!(),
        //         UserReq::Profile(_) => todo!(),
        //         UserReq::Preferences(_) => todo!(),
        //         UserReq::Password(_) => todo!(),
        //     },
        // }
        todo!()
    }
}
