mod error;
mod types;

use crate::error::Error;
use log::{debug, error, trace};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
pub use types::*;

#[derive(Clone)]
pub struct VMRestContext<'c> {
    username: &'c str,
    password: &'c str,
    endpoint: &'c str,
}

impl<'c> VMRestContext<'c> {
    pub fn new(username: &'c str, password: &'c str) -> Self {
        Self {
            username,
            password,
            endpoint: "http://127.0.0.1:8697/api",
        }
    }

    pub fn with_endpoint(mut self, endpoint: &'c str) -> Self {
        self.endpoint = endpoint;
        self
    }

    pub fn set_endpoint(&mut self, endpoint: &'c str) {
        self.endpoint = endpoint;
    }

    async fn make_request<T: DeserializeOwned>(
        &self,
        method: reqwest::Method,
        path: &[&str],
        body: Option<&str>,
        vm_password: Option<&str>,
    ) -> Result<T, Error> {
        let client = reqwest::Client::new();
        let url = format!("{}/{}", self.endpoint, path.join("/"));
        trace!("{method:?} {url:?}");
        let mut request = client
            .request(method, url)
            .basic_auth(self.username, Some(self.password))
            .header(
                reqwest::header::ACCEPT,
                "application/vnd.vmware.vmw.rest-v1+json",
            );
        if let Some(password) = vm_password {
            trace!("using vmPassword");
            request = request.query(&[("vmPassword", password)]);
        }
        if let Some(body) = body {
            trace!("body: {body:?}");
            request = request.header(
                reqwest::header::CONTENT_TYPE,
                "application/vnd.vmware.vmw.rest-v1+json",
            );
            request = request.body(body.to_owned());
        }
        let response = request.send().await?;
        let status = response.status();
        let text = response.text().await?;
        trace!("{text:?}");
        let value = serde_json::Value::from_str(&text)?;
        trace!("{value:?}");
        if status.is_client_error() {
            trace!("error code: {status}");
            let error = serde_json::from_value::<VMRestAPIError>(value)
                .inspect_err(|err| error!("{err}"))?;
            Err(error.into())
        } else {
            trace!("status: {status}");
            serde_json::from_value::<T>(value)
                .inspect_err(|err| error!("{err}"))
                .map_err(Into::into)
        }
    }

    pub async fn get_vms(&self) -> Result<Vec<VMID>, Error> {
        self.make_request(reqwest::Method::GET, &["vms"], None, None)
            .await
    }

    pub async fn get_vm_settings(
        &self,
        id: &str,
        vm_password: Option<&str>,
    ) -> Result<VMInformation, Error> {
        self.make_request(reqwest::Method::GET, &["vms", id], None, vm_password)
            .await
    }

    pub async fn get_vm_restrictions(
        &self,
        id: &str,
        vm_password: Option<&str>,
    ) -> Result<VMRestrictionsInformation, Error> {
        self.make_request(
            reqwest::Method::GET,
            &["vms", id, "restrictions"],
            None,
            vm_password,
        )
        .await
    }
    pub async fn get_vm_param_by_name(
        &self,
        id: &str,
        parameter: &str,
        vm_password: Option<&str>,
    ) -> Result<ConfigVMParamsParameter, Error> {
        self.make_request(
            reqwest::Method::GET,
            &["vms", id, "params", parameter],
            None,
            vm_password,
        )
        .await
    }

    pub async fn get_vm_power_state(
        &self,
        id: &str,
        vm_password: Option<&str>,
    ) -> Result<VMPowerState, Error> {
        let VMPowerStateWrapper { power_state } = self
            .make_request(
                reqwest::Method::GET,
                &["vms", id, "power"],
                None,
                vm_password,
            )
            .await?;
        Ok(power_state)
    }

    pub async fn get_vmnets(&self) -> Result<Networks, Error> {
        self.make_request(reqwest::Method::GET, &["vmnet"], None, None)
            .await
    }

    pub async fn set_vm_settings(
        &self,
        id: &str,
        settings: &VMParameter,
        vm_password: Option<&str>,
    ) -> Result<VMInformation, Error> {
        self.make_request(
            reqwest::Method::PUT,
            &["vms", id],
            Some(&serde_json::to_string(&settings)?),
            vm_password,
        )
        .await
    }

    pub async fn set_vm_config_parameter(
        &self,
        id: &str,
        config_parameter: &ConfigVMParamsParameter,
        vm_password: Option<&str>,
    ) -> Result<VMRestAPIResponse, Error> {
        self.make_request(
            reqwest::Method::PUT,
            &["vms", id, "configparams"],
            Some(&serde_json::to_string(&config_parameter)?),
            vm_password,
        )
        .await
    }

    pub async fn set_vm_power_state(
        &self,
        id: &str,
        state: VMPowerOperation,
        vm_password: Option<&str>,
    ) -> Result<VMPowerState, Error> {
        let VMPowerStateWrapper { power_state } = self
            .make_request(
                reqwest::Method::PUT,
                &["vms", id, "power"],
                Some(state.into()),
                vm_password,
            )
            .await?;
        Ok(power_state)
    }

    pub async fn create_vm_clone(
        &self,
        id: &str,
        clone_name: &str,
        vm_password: Option<&str>,
    ) -> Result<VMInformation, Error> {
        self.make_request(
            reqwest::Method::POST,
            &["vms"],
            Some(&serde_json::to_string(&VMCloneParameter {
                name: clone_name.to_string(),
                parent_id: id.to_string(),
            })?),
            vm_password,
        )
        .await
    }

    pub async fn create_vm_registration(
        &self,
        name: &str,
        path: &str,
        vm_password: Option<&str>,
    ) -> Result<VMRegistrationInformation, Error> {
        self.make_request(
            reqwest::Method::POST,
            &["vms", "registration"],
            Some(&serde_json::to_string(&VMRegisterParameter {
                name: name.to_string(),
                path: path.to_string(),
            })?),
            vm_password,
        )
        .await
    }

    pub async fn create_vmnet(&self, parameter: CreateVmnetParameter) -> Result<Network, Error> {
        self.make_request(reqwest::Method::POST, &["vmnets"], Some(&serde_json::to_string(&parameter)?), None)
            .await
    }

    pub async fn delete_vm(&self, id: &str, vm_password: Option<&str>) -> Result<(), Error> {
        self.make_request(reqwest::Method::DELETE, &["vms", id], None, vm_password)
            .await
    }
}
