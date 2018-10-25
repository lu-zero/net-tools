use iproute2::ConnectionHandle;
use tokio_core::reactor::Core;
use futures::Future;

pub struct NetRequest {
    pub conn: ConnectionHandle,
    pub core: Core
}

impl NetRequest {
    pub fn get(mut self) -> Vec<String> {
        let request = self.conn.link().get().execute().and_then(|links| {
            let rx = links.iter().map(|link| {
                String::from(link.name().unwrap())
            }).collect::<Vec<String>>();
            Ok(rx)
        });

        self.core.run(request).unwrap()
    }

    pub fn up(mut self, name: &str) {
        let request = self.conn.link().add().dummy(name.to_owned()).execute().and_then(|_| Ok(()));

        self.core.run(request).unwrap()
    }

    pub fn down(mut self, name: &str) {
        let request = self.conn.link().get().execute().and_then(|links| {
                let matched_link = links.iter().filter(|link| {
                    link.name().unwrap() == name
                }).next().unwrap();
                Ok(matched_link.to_owned())
            });

        let link = self.core.run(request).unwrap();
        let request = self.conn.link().del(link.index()).execute().and_then(|_| Ok(()));
        self.core.run(request).unwrap()
    }
}
