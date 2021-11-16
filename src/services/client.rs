//! 以 awc 為基礎的用戶端工具服務。
use std::io::Write;

use awc::error::{PayloadError, SendRequestError};
use awc::{Client, SendClientRequest};
use futures_util::{FutureExt, StreamExt};
use http::Uri;
use log::{debug, trace};
use thiserror::Error;

use crate::types::Output;

/// 從指定 URL 取得資源。
///
/// - client: [`awc::Client`] 實體。
/// - url: 要取得的資源 URI。
pub async fn get_resource(client: &Client, uri: &Uri) -> SendClientRequest {
    debug!("getting sources from: {}", uri);
    client.get(uri).send()
}

/// 將回傳資料逐 chunk 寫入檔案。
///
/// - request: [`get_resource`] 的回傳資料。
/// - output: 要輸出的位置。
pub async fn write_chunks_to_file(
    request: &mut SendClientRequest,
    output: &mut Output,
) -> ClientServiceResult<()> {
    /* Get stream of <request> body */
    trace!("transforming the request to stream");
    let mut stream = {
        let mut response = request.await?;
        response.body().into_stream()
    };

    /* Get the File reference of <output> */
    let file = output.get_file_mut();

    trace!("writing the chunk in stream to the file");
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk)?;
    }

    Ok(())
}

/// 用戶端服務的錯誤。
#[derive(Error, Debug)]
pub enum ClientServiceError {
    /// [`SendRequestError`] 錯誤的封裝。
    #[error("failed to fetch resource: {0}")]
    FetchResourceError(#[from] SendRequestError),

    /// [`PayloadError`] 錯誤的封裝。
    #[error("payload error: {0}")]
    PayloadError(#[from] PayloadError),

    /// [`std::io::Error`] 錯誤的封裝。
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}

/// 用戶端服務的回傳值。
///
/// 錯誤的詳細資料請見 [`ClientServiceError`]。
pub type ClientServiceResult<T> = Result<T, ClientServiceError>;

#[cfg(test)]
mod tests {
    use std::io::{Read, Seek, SeekFrom};

    use super::*;

    #[actix_web::test]
    async fn get_resource_test() {
        let client = awc::Client::new();
        let uri = Uri::from_static("https://www.google.com");
        let resp = get_resource(&client, &uri).await;
        let resp = resp.await.unwrap();

        assert_eq!(resp.status(), 200);
    }

    #[actix_web::test]
    async fn write_chunks_to_file_test() {
        let client = awc::Client::new();
        let uri = Uri::from_static("https://www.example.com");

        let normal_way = {
            let resp = get_resource(&client, &uri).await;
            let mut resp = resp.await.unwrap();
            let bytes = resp.body().await.unwrap();

            String::from_utf8_lossy(&bytes).to_string()
        };

        let write_chunk_way = {
            let mut buf = String::new();
            let mut output = Output(tempfile::tempfile().unwrap());
            let mut resp = get_resource(&client, &uri).await;
            write_chunks_to_file(&mut resp, &mut output).await.unwrap();

            let f = output.get_file_mut();
            f.seek(SeekFrom::Start(0)).unwrap();
            f.read_to_string(&mut buf).unwrap();

            buf
        };

        assert_eq!(normal_way, write_chunk_way);
    }
}
