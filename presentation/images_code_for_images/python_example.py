##### Example 1 #####

def work_on_array(arr, idx_start=None, idx_end=None):
    """ Quick example of how duck typing can go wrong. """
    if idx_start and idx_end:
        return np.sum(arr[idx_start:idx_end])
    return np.sum(arr)

assert work_on_array([1, 1, 1, 1, 1], start=0, end=3) == 5

##### Example 2 #####

def inner_product(vec1, vec2):
    return np.sum(vec1 * vec2)

vec1, vec2 = np.ones(10), np.ones(10)
assert inner_product(vec1, vec2) == 10.0
# Outer product by mistake
assert inner_product(vec1, vec2.reshape(-1, 1)) == 100.0

