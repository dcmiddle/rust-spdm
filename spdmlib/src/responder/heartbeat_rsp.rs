// Copyright (c) 2020 Intel Corporation
//
// SPDX-License-Identifier: Apache-2.0

use crate::common::SpdmCodec;
use crate::error::SpdmResult;
use crate::message::*;
use crate::responder::*;

impl ResponderContext {
    pub fn handle_spdm_heartbeat<'a>(
        &mut self,
        session_id: u32,
        bytes: &[u8],
        writer: &'a mut Writer,
    ) -> (SpdmResult, Option<&'a [u8]>) {
        self.write_spdm_heartbeat_response(session_id, bytes, writer);

        (Ok(()), Some(writer.used_slice()))
    }

    pub fn write_spdm_heartbeat_response(
        &mut self,
        session_id: u32,
        bytes: &[u8],
        writer: &mut Writer,
    ) {
        let mut reader = Reader::init(bytes);
        let message_header = SpdmMessageHeader::read(&mut reader);
        if let Some(message_header) = message_header {
            if message_header.version != self.common.negotiate_info.spdm_version_sel {
                self.write_spdm_error(SpdmErrorCode::SpdmErrorVersionMismatch, 0, writer);
                return;
            }
        } else {
            self.write_spdm_error(SpdmErrorCode::SpdmErrorInvalidRequest, 0, writer);
            return;
        }

        self.common.reset_buffer_via_request_code(
            SpdmRequestResponseCode::SpdmRequestHeartbeat,
            Some(session_id),
        );

        let heartbeat_req = SpdmHeartbeatRequestPayload::spdm_read(&mut self.common, &mut reader);
        if let Some(heartbeat_req) = heartbeat_req {
            debug!("!!! heartbeat req : {:02x?}\n", heartbeat_req);
        } else {
            error!("!!! heartbeat req : fail !!!\n");
            self.write_spdm_error(SpdmErrorCode::SpdmErrorInvalidRequest, 0, writer);
            return;
        }

        info!("send spdm heartbeat rsp\n");

        let response = SpdmMessage {
            header: SpdmMessageHeader {
                version: self.common.negotiate_info.spdm_version_sel,
                request_response_code: SpdmRequestResponseCode::SpdmResponseHeartbeatAck,
            },
            payload: SpdmMessagePayload::SpdmHeartbeatResponse(SpdmHeartbeatResponsePayload {}),
        };
        let res = response.spdm_encode(&mut self.common, writer);
        if res.is_err() {
            self.write_spdm_error(SpdmErrorCode::SpdmErrorUnspecified, 0, writer);
        }
    }
}
