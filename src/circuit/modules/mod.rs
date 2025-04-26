///
pub mod poseidon;

///
pub mod polycommit;

///
pub mod planner;

///
pub mod errors;

use halo2_proofs::{circuit::Layouter, plonk::ConstraintSystem};
use halo2curves::ff::PrimeField;
pub use planner::*;

use crate::tensor::{TensorType, ValTensor};

use super::region::ConstantsMap;

/// Module trait used to extend ezkl functionality
pub trait Module<F: PrimeField + TensorType + PartialOrd> {
    /// Config
    type Config;
    /// The return type after an input assignment
    type InputAssignments;
    /// The inputs used in the run function
    type RunInputs;
    /// The params used in configure
    type Params;

    /// construct new module from config
    fn new(config: Self::Config) -> Self;
    /// Configure
    fn configure(meta: &mut ConstraintSystem<F>, params: Self::Params) -> Self::Config;
    /// Name
    fn name(&self) -> &'static str;
    /// Run the operation the module represents
    fn run(input: Self::RunInputs) -> Result<Vec<Vec<F>>, errors::ModuleError>;
    /// Layout inputs
    fn layout_inputs(
        &self,
        layouter: &mut impl Layouter<F>,
        input: &[ValTensor<F>],
        constants: &mut ConstantsMap<F>,
    ) -> Result<Self::InputAssignments, errors::ModuleError>;
    /// Layout
    fn layout(
        &self,
        layouter: &mut impl Layouter<F>,
        input: &[ValTensor<F>],
        row_offset: usize,
        constants: &mut ConstantsMap<F>,
    ) -> Result<ValTensor<F>, errors::ModuleError>;
    /// Number of instance values the module uses every time it is applied
    fn instance_increment_input(&self) -> Vec<usize>;
    /// Number of rows used by the module
    fn num_rows(input_len: usize) -> usize;
    /// Optimize the module for proving times and memory usage while maintaining soundness
    fn optimize(&self, params: Self::Params) -> Result<(), errors::ModuleError>;
}

/// Optimize the proof system for proving times and memory usage while maintaining soundness
pub fn optimize_proof_system<F: PrimeField + TensorType + PartialOrd>(
    module: &impl Module<F>,
    params: <impl Module<F> as Module<F>>::Params,
) -> Result<(), errors::ModuleError> {
    module.optimize(params)
}
