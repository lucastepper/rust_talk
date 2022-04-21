import numpy as np
import rust_python


array1 = np.array([1., 2., 3., 4., 5.])
rust_python.increment_array(array1, 1.5)
np.testing.assert_allclose(array1, np.array([1., 2., 3., 4., 5.]) + 1.5)
print(array1)


array1 = np.array([1., 2., 3., 4., 5.])
array2 = np.array([2., 3., 4., 5., 6.])
output = rust_python.mult_arrays(array1, array2)
np.testing.assert_allclose(output, array1 * array2)
print(output)
