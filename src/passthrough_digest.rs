use digest::{consts::U32, generic_array::GenericArray, BlockInput, FixedOutput, Reset, Update};
use std::convert::TryInto;

#[derive(Clone, Default)]
pub struct PassthroughDigest {
    value: [u8; 32],
}

impl Update for PassthroughDigest {
    fn update(&mut self, data: impl AsRef<[u8]>) {
        let d = data.as_ref();
        self.value = d.try_into().unwrap();
    }
}

impl FixedOutput for PassthroughDigest {
    type OutputSize = U32;

    fn finalize_into(self, out: &mut GenericArray<u8, Self::OutputSize>) {
        *out = self.value.into();
    }

    fn finalize_into_reset(&mut self, out: &mut GenericArray<u8, Self::OutputSize>) {
        *out = self.value.into();
    }
}

impl Reset for PassthroughDigest {
    fn reset(&mut self) {}
}

impl BlockInput for PassthroughDigest {
    type BlockSize = U32;
}
