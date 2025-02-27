// Copyright (c) 2020 Intel Corporation
//
// SPDX-License-Identifier: Apache-2.0

use crate::common::SpdmCodec;
use crate::error::SpdmResult;
use crate::message::*;
use crate::protocol::SpdmVersion;
use crate::responder::*;

impl ResponderContext {
    pub fn write_spdm_error(
        &mut self,
        error_code: SpdmErrorCode,
        error_data: u8,
        writer: &mut Writer,
    ) {
        let error = SpdmMessage {
            header: SpdmMessageHeader {
                version: if self.common.negotiate_info.spdm_version_sel.get_u8() == 0 {
                    SpdmVersion::SpdmVersion10
                } else {
                    self.common.negotiate_info.spdm_version_sel
                },
                request_response_code: SpdmRequestResponseCode::SpdmResponseError,
            },
            payload: SpdmMessagePayload::SpdmErrorResponse(SpdmErrorResponsePayload {
                error_code,
                error_data,
                extended_data: SpdmErrorResponseExtData::SpdmErrorExtDataNone(
                    SpdmErrorResponseNoneExtData {},
                ),
            }),
        };
        writer.clear();
        let _ = error.spdm_encode(&mut self.common, writer);
    }
}

impl ResponderContext {
    pub fn handle_error_request<'a>(
        &mut self,
        error_code: SpdmErrorCode,
        bytes: &[u8],
        writer: &'a mut Writer,
    ) -> (SpdmResult, Option<&'a [u8]>) {
        self.write_error_response(error_code, bytes, writer);

        (Ok(()), Some(writer.used_slice()))
    }

    pub fn write_error_response(
        &mut self,
        error_code: SpdmErrorCode,
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
            let error_data = if error_code == SpdmErrorCode::SpdmErrorUnsupportedRequest {
                message_header.request_response_code.get_u8()
            } else {
                0u8
            };
            self.write_spdm_error(error_code, error_data, writer);
        } else {
            self.write_spdm_error(SpdmErrorCode::SpdmErrorInvalidRequest, 0, writer);
        }
    }
}
