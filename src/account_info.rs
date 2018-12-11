use crate::{PyAccountId, PyClaim, PyPublicKey};
use derive_more::From;
use hedera::AccountInfo;
use itertools::Itertools;
use pyo3::prelude::*;

#[pyclass(name = AccountInfo)]
#[derive(From)]
pub struct PyAccountInfo {
    pub(crate) inner: AccountInfo,
}

#[pymethods]
impl PyAccountInfo {
    #[getter]
    fn account_id(&self) -> PyResult<String> {
        Ok(self.inner.account_id.to_string())
    }

    #[getter]
    fn contract_account_id(&self) -> PyResult<String> {
        Ok(self.inner.contract_account_id.to_string())
    }

    #[getter]
    fn deleted(&self) -> PyResult<bool> {
        Ok(self.inner.deleted as bool)
    }

    #[getter]
    fn proxy_account_id(&self) -> PyResult<Option<PyAccountId>> {
        Ok(self.inner.proxy_account_id.map(Into::into))
    }

    #[getter]
    fn proxy_fraction(&self) -> PyResult<i32> {
        Ok(self.inner.proxy_fraction as i32)
    }

    #[getter]
    fn proxy_received(&self) -> PyResult<i64> {
        Ok(self.inner.proxy_received as i64)
    }

    #[getter]
    fn key(&self) -> PyResult<PyPublicKey> {
        Ok(self.inner.key.clone().into())
    }

    #[getter]
    fn balance(&self) -> PyResult<u64> {
        Ok(self.inner.balance as u64)
    }

    #[getter]
    fn generate_send_record_threshold(&self) -> PyResult<u64> {
        Ok(self.inner.generate_send_record_threshold as u64)
    }

    #[getter]
    fn generate_receive_record_threshold(&self) -> PyResult<u64> {
        Ok(self.inner.generate_receive_record_threshold as u64)
    }

    #[getter]
    fn receiver_signature_required(&self) -> PyResult<bool> {
        Ok(self.inner.receiver_signature_required as bool)
    }

    // fn get_expiration_time(&self, py: Python) -> PyResult<Py<PyDateTime>> {
    //     py_date_time(self.inner.expiration_time, py)
    // }

    // fn get_auto_renew_period(&self, py: Python) -> PyResult<Py<PyDelta>> {
    //     let renew_period = self.inner.auto_renew_period;
    //     let seconds = renew_period.as_secs() as i32;
    //     let microseconds = renew_period.subsec_micros() as i32;

    //     PyDelta::new(py, 0, seconds, microseconds, false)
    // }

    #[getter]
    fn claims(&self) -> PyResult<Vec<PyClaim>> {
        let claims = self.inner.claims.clone().into_iter().map_into().collect();

        Ok(claims)
    }
}