use std::pin::Pin;

use libsql_replication::replicator::{ReplicatorClient, Error};
use libsql_replication::meta::WalIndexMeta;
use libsql_replication::frame::{FrameNo, Frame};
use libsql_replication::rpc::replication::{HelloRequest, LogOffset};
use tokio::sync::watch;
use tokio_stream::Stream;

struct Client {
    remote: super::client::Client,
    current_fetch_fno: Option<FrameNo>,
    pub(crate) current_commit_fno_notifier: watch::Sender<Option<FrameNo>>,
    meta: WalIndexMeta,
}

impl Client {
    fn next_offset(&self) -> FrameNo {
        todo!()
    }
}

#[async_trait::async_trait]
impl ReplicatorClient for Client {
    type FrameStream = Pin<Box<dyn Stream<Item = Result<libsql_replication::frame::Frame, Error>> + Send + 'static>>;

    /// Perform handshake with remote
    async fn handshake(&mut self) -> Result<(), Error> {
        tracing::info!("Attempting to perform handshake with primary.");
        match self.remote.replication.hello(HelloRequest::default()).await {
            Ok(resp) => {
                let hello = resp.into_inner();
                self.meta.merge_hello(hello)?;
                self.current_commit_fno_notifier
                    .send_replace(self.meta.current_frame_no());

                Ok(())
            }
            Err(e) => Err(Error::Client(e.into()))?,
        }
    }

    /// Return a stream of frames to apply to the database
    async fn next_frames(&mut self) -> Result<Self::FrameStream, Error> {
        let frames = self
            .remote
            .replication
            .batch_log_entries(LogOffset { next_offset: self.next_offset() })
            .await
            .map_err(|e| Error::Client(e.into()))?
            .into_inner()
            .frames;

        let frames_iter = frames
            .into_iter()
            .map(|f| Frame::try_from(&*f.data).map_err(|e| Error::Client(e.into())));

        Ok(Box::pin(tokio_stream::iter(frames)))
    }

    /// Return a snapshot for the current replication index. Called after next_frame has returned a
    /// NeedSnapshot error
    async fn snapshot(&mut self) -> Result<Self::FrameStream, Error> {
        todo!();
    }
    /// set the new commit frame_no
    async fn commit_frame_no(&mut self, frame_no: FrameNo) -> Result<(), Error> {
        todo!()
    }
}

