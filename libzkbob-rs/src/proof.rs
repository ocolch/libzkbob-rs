use libzeropool::{
    circuit::{
        delegated_deposit::check_delegated_deposit_batch, tree::tree_update, tx::c_transfer,
    },
    fawkes_crypto::{
        backend::bellman_groth16::engines::Engine,
        backend::bellman_groth16::{prover::{prove, Proof}, PrecomputedData},
        backend::bellman_groth16::{Parameters, prover::prove_precomputed},
        ff_uint::Num,
    },
    native::{
        delegated_deposit::{DelegatedDepositBatchPub, DelegatedDepositBatchSec},
        params::PoolParams,
        tree::{TreePub, TreeSec},
        tx::{TransferPub, TransferSec},
    },
};

pub fn prove_tx<P, E>(
    params: &Parameters<E>,
    pool_params: &P,
    transfer_pub: TransferPub<E::Fr>,
    transfer_sec: TransferSec<E::Fr>,
    precomputed: &Option<PrecomputedData<E::Fr>>,
) -> (Vec<Num<E::Fr>>, Proof<E>)
where
    P: PoolParams<Fr = E::Fr>,
    E: Engine,
{
    let circuit = |public, secret| {
        c_transfer(&public, &secret, pool_params);
    };

    match precomputed {
        Some(precomputed) => {
            prove_precomputed(params, &transfer_pub, &transfer_sec, circuit, precomputed)
        },
        None => prove(params, &transfer_pub, &transfer_sec, circuit)
    }
}

pub fn prove_tree<P, E>(
    params: &Parameters<E>,
    pool_params: &P,
    tree_pub: TreePub<E::Fr>,
    tree_sec: TreeSec<E::Fr>,
    precomputed: &Option<PrecomputedData<E::Fr>>,
) -> (Vec<Num<E::Fr>>, Proof<E>)
where
    P: PoolParams<Fr = E::Fr>,
    E: Engine,
{
    let circuit = |public, secret| {
        tree_update(&public, &secret, pool_params);
    };

    match precomputed {
        Some(precomputed) => {
            prove_precomputed(params, &tree_pub, &tree_sec, circuit, precomputed)
        },
        None => prove(params, &tree_pub, &tree_sec, circuit)
    }
}

pub fn prove_delegated_deposit<P, E>(
    params: &Parameters<E>,
    pool_params: &P,
    d_pub: DelegatedDepositBatchPub<E::Fr>,
    d_sec: DelegatedDepositBatchSec<E::Fr>,
    precomputed: &Option<PrecomputedData<E::Fr>>,
) -> (Vec<Num<E::Fr>>, Proof<E>)
where
    P: PoolParams<Fr = E::Fr>,
    E: Engine,
{
    let circuit = |public, secret| {
        check_delegated_deposit_batch(&public, &secret, pool_params);
    };

    match precomputed {
        Some(precomputed) => {
            prove_precomputed(params, &d_pub, &d_sec, circuit, precomputed)
        },
        None => prove(params, &d_pub, &d_sec, circuit)
    }
}