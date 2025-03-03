"""
This module provides a set of functions for creating data.
It is designed to generate and form data in various formats.

"""
from typing import Sequence

import numpy as np
from numpy.typing import NDArray


def g_list_arrays_indeces_from_array_bool(
    arr: NDArray[np.bool],
    check_args_type: bool = True,
) -> list[NDArray[np.int32]]:
    """
    Splits a boolean array into ranges of indices of
    contiguous sequences of True.

    Args:
        arr (NDArray[np.bool]): The boolean array to process.
        check_args_type (bool, optional):
        Whether to check the type of arguments. Defaults to True.

    Returns:
        list[NDArray[np.int32]]: A list of arrays
            containing indices of contiguous sequences of True.
    """
    if check_args_type:
        assert isinstance(arr, np.ndarray) and arr.dtype == np.bool

    lst: list[NDArray[np.int32]] = []
    num1 = 0
    bool_ = False

    for i, el in enumerate(arr):
        if el:
            if not bool_ and i == len(arr) - 1:
                num1 = i
                lst.append(np.arange(i, i + 1))
            elif not bool_:
                bool_ = True
                num1 = i
            elif bool_ and i == len(arr) - 1:
                lst.append(np.arange(i, i + 1))
        else:
            if bool_:
                bool_ = False
                lst.append(np.arange(num1, i))
    return lst

def g_comparison(
    collection: np.ndarray,
    check_transf: bool = True,
) -> np.bool:
    """
    Checks if all elements in the collection
    are equal to the first element.

    Args:
        collection (np.ndarray): The array to compare elements in.
        check_transf (bool, optional): Whether to convert the input
            to a numpy array if necessary. Defaults to True.

    Returns:
        np.bool: True if all elements are equal to the first one
        (or if the collection is empty), False otherwise.
    """
    if check_transf:
        if not isinstance(collection, np.ndarray):
            collection = np.array(collection)

    if collection.size > 0:
        return np.all([collection[0] == collection])
    return np.bool(True)

def g_count_mean(
    v: Sequence,
) -> np.int64:
    """
    returns mean size number of elements

    Args:
        Sequence (v): matrix collection
    Returns:
        np.int64: mean number of elements in the collections
    """
    return np.int64(np.mean([len(el) for el in v]))

def g_arrays_bool_fill_gaps(
    arr_1: NDArray[np.bool],
    arr_2: NDArray[np.bool],
    use_copy=True,
    check_args_transform=True,
) -> tuple[NDArray[np.bool], NDArray[np.bool]]:
    """
    completely fills True values without gaps with False state.
    // This can be used when filling long and short
    boolean arrays to fill gaps where both arrays are False.

    Args:
        NDArray (arr_1): boolean array
        NDArray (arr_2): boolean array
        bool (check_args_transform): check if transform arguments are valid
    Returns:
        tuple[NDArray[np.bool], NDArray[np.bool]]: filled arrays
             with gaps filled with False
    """
    if check_args_transform:
        if not isinstance(arr_1, np.ndarray):
            arr_1 = np.array(arr_1)
        if not isinstance(arr_2, np.ndarray):
            arr_2 = np.array(arr_2)

    bools = arr_1 + arr_2
    s_l = np.nan
    l_l = np.nan

    if use_copy:
        arr_1 = arr_1.copy()
        arr_2 = arr_2.copy()
    for i, (m, s, l) in enumerate(zip(bools, arr_1, arr_2)):
        if s:
            s_l = i
        elif l:
            l_l = i
        if not m:
            if s_l > l_l:
                arr_1[i] = True
            else:
                arr_2[i] = True
    return arr_1, arr_2

def g_slice_or_i(
    lst: Sequence,
    check_args_type: bool = True,
) -> slice | int:
    """
    returns a slice or index for indexing sequences

    Args:
        Sequence (lst): sequence
        bool (check_args_type): check if type of arguments are valid
    Returns:
        slice | int: value used as indexing
    """
    if check_args_type:
        assert isinstance(lst, Sequence), "lst should be a sequence"

    len_lst = len(lst)

    if len_lst > 1:
        return slice(*lst)
    elif len_lst > 0:
        return lst[-1]
    else:
        return slice(None, None)
