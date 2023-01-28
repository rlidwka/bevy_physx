use crate::{
    DeriveClassForNewType,
    traits::Class,
};

use physx_sys::{PxVehicleKeySmoothingData, PxVehiclePadSmoothingData, phys_PxVehicleDriveTankSmoothDigitalRawInputsAndSetAnalogInputs, phys_PxVehicleDriveTankSmoothAnalogRawInputsAndSetAnalogInputs, phys_PxVehicleDriveNWSmoothDigitalRawInputsAndSetAnalogInputs, PxFixedSizeLookupTable_8_, phys_PxVehicleDriveNWSmoothAnalogRawInputsAndSetAnalogInputs};

use super::{
    PxVehicleDriveDynData,
    VehicleDriveDynData,
    VehicleDriveTank,
    VehicleDriveTankRawInputData, VehicleDriveNW, VehicleDriveNWRawInputData,
};

#[derive(Clone)]
pub struct VehicleKeySmoothingData {
    obj: PxVehicleKeySmoothingData,
}

DeriveClassForNewType!(VehicleKeySmoothingData: PxVehicleKeySmoothingData);

impl VehicleKeySmoothingData {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_rise_rates(&mut self, rise_rates: &[f32]) {
        self.obj.mRiseRates[..rise_rates.len()].copy_from_slice(rise_rates);
    }

    pub fn set_fall_rates(&mut self, fall_rates: &[f32]) {
        self.obj.mFallRates[..fall_rates.len()].copy_from_slice(fall_rates);
    }
}

impl Default for VehicleKeySmoothingData {
    fn default() -> Self {
        Self {
            obj: PxVehicleKeySmoothingData {
                mRiseRates: [0.; PxVehicleDriveDynData::MAX_NB_ANALOG_INPUTS],
                mFallRates: [0.; PxVehicleDriveDynData::MAX_NB_ANALOG_INPUTS],
            }
        }
    }
}

#[derive(Clone)]
pub struct VehiclePadSmoothingData {
    obj: PxVehiclePadSmoothingData,
}

DeriveClassForNewType!(VehiclePadSmoothingData: PxVehiclePadSmoothingData);

impl VehiclePadSmoothingData {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_rise_rates(&mut self, rise_rates: &[f32]) {
        self.obj.mRiseRates[..rise_rates.len()].copy_from_slice(rise_rates);
    }

    pub fn set_fall_rates(&mut self, fall_rates: &[f32]) {
        self.obj.mFallRates[..fall_rates.len()].copy_from_slice(fall_rates);
    }
}

impl Default for VehiclePadSmoothingData {
    fn default() -> Self {
        Self {
            obj: PxVehiclePadSmoothingData {
                mRiseRates: [0.; PxVehicleDriveDynData::MAX_NB_ANALOG_INPUTS],
                mFallRates: [0.; PxVehicleDriveDynData::MAX_NB_ANALOG_INPUTS],
            }
        }
    }
}

#[derive(Default, Clone)]
pub struct VehicleSteerVsForwardSpeedTable {
    data_pairs: [f32; Self::MAX_PAIRS * 2],
    nb_data_pairs: usize,
}

impl VehicleSteerVsForwardSpeedTable {
    const MAX_PAIRS: usize = 8;
}

impl VehicleSteerVsForwardSpeedTable {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_data(&mut self, data: &[(f32, f32)]) {
        debug_assert!(data.len() <= Self::MAX_PAIRS);
        self.nb_data_pairs = data.len();

        for idx in 0..Self::MAX_PAIRS / 2 {
            let pair = data.get(idx).unwrap_or(&(std::f32::MAX, std::f32::MAX));
            self.data_pairs[idx * 2] = pair.0;
            self.data_pairs[idx * 2 + 1] = pair.1;
        }
    }
}

impl From<VehicleSteerVsForwardSpeedTable> for PxFixedSizeLookupTable_8_ {
    fn from(value: VehicleSteerVsForwardSpeedTable) -> Self {
        Self {
            mDataPairs: value.data_pairs,
            mNbDataPairs: value.nb_data_pairs as u32,
            mPad: Default::default(),
        }
    }
}

impl<T: VehicleDriveNW> VehicleDriveNWControl for T {}

pub trait VehicleDriveNWControl: VehicleDriveNW {
    fn smooth_digital_raw_inputs_and_set_analog_inputs(
        &mut self,
        steer_vs_forward_speed_table: &VehicleSteerVsForwardSpeedTable,
        key_smoothing: &VehicleKeySmoothingData,
        raw_input_data: &impl VehicleDriveNWRawInputData,
        timestep: f32,
        is_vehicle_in_air: bool,
    ) {
        let steer_vs_forward_speed_table = PxFixedSizeLookupTable_8_::from(steer_vs_forward_speed_table.clone());

        unsafe {
            phys_PxVehicleDriveNWSmoothDigitalRawInputsAndSetAnalogInputs(
                key_smoothing.as_ptr(),
                &steer_vs_forward_speed_table,
                raw_input_data.as_ptr(),
                timestep,
                is_vehicle_in_air,
                self.as_mut_ptr(),
            );
        }
    }

    fn smooth_analog_raw_inputs_and_set_analog_inputs(
        &mut self,
        steer_vs_forward_speed_table: &VehicleSteerVsForwardSpeedTable,
        pad_smoothing: &VehiclePadSmoothingData,
        raw_input_data: &impl VehicleDriveNWRawInputData,
        timestep: f32,
        is_vehicle_in_air: bool,
    ) {
        let steer_vs_forward_speed_table = PxFixedSizeLookupTable_8_::from(steer_vs_forward_speed_table.clone());

        unsafe {
            phys_PxVehicleDriveNWSmoothAnalogRawInputsAndSetAnalogInputs(
                pad_smoothing.as_ptr(),
                &steer_vs_forward_speed_table,
                raw_input_data.as_ptr(),
                timestep,
                is_vehicle_in_air,
                self.as_mut_ptr(),
            );
        }
    }
}

impl<T: VehicleDriveTank> VehicleDriveTankControl for T {}

pub trait VehicleDriveTankControl: VehicleDriveTank {
    fn smooth_digital_raw_inputs_and_set_analog_inputs(
        &mut self,
        key_smoothing: &VehicleKeySmoothingData,
        raw_input_data: &impl VehicleDriveTankRawInputData,
        timestep: f32,
    ) {
        unsafe {
            phys_PxVehicleDriveTankSmoothDigitalRawInputsAndSetAnalogInputs(
                key_smoothing.as_ptr(),
                raw_input_data.as_ptr(),
                timestep,
                self.as_mut_ptr(),
            );
        }
    }

    fn smooth_analog_raw_inputs_and_set_analog_inputs(
        &mut self,
        pad_smoothing: &VehiclePadSmoothingData,
        raw_input_data: &impl VehicleDriveTankRawInputData,
        timestep: f32,
    ) {
        unsafe {
            phys_PxVehicleDriveTankSmoothAnalogRawInputsAndSetAnalogInputs(
                pad_smoothing.as_ptr(),
                raw_input_data.as_ptr(),
                timestep,
                self.as_mut_ptr(),
            );
        }
    }
}