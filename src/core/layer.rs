pub struct Layer<'a, const N: usize> {
    input: [f32;N],
    weights: [[f32;N];N],
    biasses: [f32;N],
    output: [f32;N],
    act: &'a dyn Fn(f32) -> f32
}


impl <'a, const N: usize> Layer<'a, N> {
    pub fn setup(weights: [[f32;N];N], biasses: [f32;N], act: &dyn Fn(f32)->f32) -> Layer<N> {

        Layer::<N> {
            input: [0.0;N],
            weights: weights,
            biasses: biasses,
            output: [0.0;N],
            act: act
        }
    }

    pub fn feed(&mut self, inputs: [f32;N]) {
        self.input = inputs;
    }
}