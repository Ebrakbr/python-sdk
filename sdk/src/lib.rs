// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the Aleo SDK library.

// The Aleo SDK library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Aleo SDK library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Aleo SDK library. If not, see <https://www.gnu.org/licenses/>.

use pyo3::prelude::*;

mod account;
mod coinbase;
mod types;

use account::*;
use coinbase::*;

#[pymodule]
#[pyo3(name = "aleo")]
fn register_module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Account>()?;
    m.add_class::<Address>()?;
    m.add_class::<CoinbasePuzzle>()?;
    m.add_class::<CoinbaseVerifyingKey>()?;
    m.add_class::<ComputeKey>()?;
    m.add_class::<EpochChallenge>()?;
    m.add_class::<PrivateKey>()?;
    m.add_class::<ProverSolution>()?;
    m.add_class::<RecordCiphertext>()?;
    m.add_class::<RecordPlaintext>()?;
    m.add_class::<Signature>()?;
    m.add_class::<ViewKey>()?;
    Ok(())
}
