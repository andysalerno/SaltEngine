use futures::{AsyncRead, AsyncWrite};

use crate::websocket_server::SharedContext;
use crate::{connection::Connection, websocket_server::Result};

async fn handle_connection<S>(mut connection: Connection, context: SharedContext) -> Result<()>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    todo!()
}
