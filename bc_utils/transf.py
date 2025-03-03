"""
This module provides a set of functions for transforming data.
It is designed to process and convert data into various formats.

"""

from collections.abc import Iterable
from typing import (
    Callable,
    Sequence,
    Any,
    Coroutine,
)

import numpy as np


def g_not_iter_from_iter(
    iter_: Iterable,
    use_ignore_str: bool = False,
    check_args: bool = True,
    check_args_type: bool = True,
) -> list:
    """
    Returns all values from nested iterables

    Args:
        Iterable (iter_): iterable value
        bool (use_ignore_str): if True, excludes strings from the result
    Returns:
        list: list of values
    """
    if check_args:
        if check_args_type:
            assert isinstance(iter_, Iterable)

    not_iter = []

    def from_iter(iter__=iter_):
        for item in iter__:
            item = item.values() if isinstance(item, dict) else item
            if isinstance(item, Iterable) and not isinstance(item, str):
                from_iter(item)
            else:
                not_iter.append(item)
    from_iter(iter_)
    return [
        el
        for el in not_iter
        if not isinstance(el, str) if use_ignore_str
    ]

def g_rolling(
    src: np.ndarray,
    window: int,
    func: Callable,
    check_args: bool = True,
    check_args_transform: bool = True,
) -> np.ndarray:
    """
    Applies a function to a rolling window of an array

    Args:
        np.ndarray (src): source array
        int (window): size of rolling window
        Callable (func): function to apply to rolling window
        bool (check_args): check if arguments are valid
        bool (check_args_transform): check if transform arguments are valid
    Returns:
        np.ndarray: array with rolling window results
    """
    if check_args:
        if check_args_transform:
            if not isinstance(src, np.ndarray):
                src = np.array(src)

    return np.array([
        func(src[i - window:i]) if i >= window else np.nan
        for i in range(src.size)
    ])

def g_dict_choice(
    dct: dict,
    ignore: frozenset = frozenset(("",)),
    need: frozenset = frozenset({}),
    error_handling: bool = False,
    check_args: bool = True,
    check_args_type: bool = True,
    check_args_transform: bool = True,
) -> dict:
    """
    Creates a dictionary with the selected keys
    from the passed dictionary

    Args:
        dict (dct): nested dictionary
        frozenset (ignore): keys to exclude from the result
        frozenset (need): keys to include in the result
        bool (error_handling): if True, returns None for missing keys
        bool (check_args): check if arguments are valid
        bool (check_args_type): check if type of arguments is valid
        bool (check_args_transform): check if transform arguments are valid
    Returns:
        dict: selected keys
    """
    if check_args:
        if check_args_type:
            assert isinstance(dct, dict), "dct != dict type"
        if check_args_transform:
            if not isinstance(ignore, frozenset) and isinstance(ignore, str):
                ignore = frozenset({ignore})
            elif not isinstance(ignore, frozenset):
                ignore = frozenset(ignore)
            if not need:
                need = frozenset(dct.keys())
            elif not isinstance(need, frozenset) and isinstance(need, str):
                need = frozenset({need})
            elif not isinstance(need, frozenset):
                need = frozenset(need)
    return {
        k: (dct[k] if not error_handling else dct.get(k))
        for k in dct
        if k not in ignore and k in need
    }

def g_dict_choice2(
    dct: dict,
    ignore: frozenset = frozenset({""}),
    need: frozenset = frozenset({}),
    error_handling: bool = False,
    check_args: bool = True,
    check_args_type: bool = True,
    check_args_transform: bool = True,
):
    """
    Creates a dictionary with the selected keys
    from the passed nested dictionary

    Args:
        dict (dct): nested dictionary
        frozenset (ignore): keys to exclude from the result
        frozenset (need): keys to include in the result
        bool (error_handling): if True, returns None for missing keys
        bool (check_args): check if arguments are valid
        bool (check_args_type): check if type of arguments is valid
        bool (check_args_transform): check if transform arguments are valid
    Returns:
        dict: selected keys
    """
    if check_args:
        if check_args_type:
            assert isinstance(dct, dict), "dct != dict type"
        if check_args_transform:
            if not isinstance(ignore, frozenset) and isinstance(ignore, str):
                ignore = frozenset({ignore})
            elif not isinstance(ignore, frozenset):
                ignore = frozenset(ignore)
            if not need:
                need = frozenset(dct.keys())
            elif not isinstance(need, frozenset) and isinstance(need, str):
                need = frozenset({need})
            elif not isinstance(need, frozenset):
                need = frozenset(need)
    return {
        k: g_dict_choice(
            v,
            ignore=ignore,
            need=need,
            error_handling=error_handling,
        )
        for k, v in dct.items()
    }

def g_dict_extract2(
    dct: dict,
    key: str,
    check_args_type: bool = True,
) -> dict:
    """
    Create a dictionary containing the choice key

    Args:
        dict (dct): dictionary
        str (key): key to extract
    Returns:
        dict: dictionary
    """
    if check_args_type:
        assert isinstance(dct, dict)
        assert isinstance(key, str)

    return {
        k: v[key]
        for k, v in dct.items()
    }

def g_iloc_coll(
    obj: Sequence,
    slice_or_i: slice | int,
    error_handling: bool = False,
    v: Sequence = (),
    check_args_type: bool = True,
) -> Sequence:
    """
    Returns a Sequence containing choices elements

    Args:
        Sequence (obj): Sequence
        slice | int (slice_or_i): slice or index
        bool (error_handling): if True, raises an exception for invalid index
        int (v): default value for invalid index
        bool (check_args_type): check if type of arguments is valid
    Returns:
        Sequence: the collection
    """
    if check_args_type:
        assert isinstance(slice_or_i, (slice, int)), (
            "slice_or_i should be a slice or an integer"
        )

    if isinstance(obj, Iterable) and not isinstance(obj, str):
        if error_handling:
            try:
                return obj[slice_or_i]
            except IndexError:
                return v
        else:
            return obj[slice_or_i]
    return obj

def g_iloc_coll2(
    obj: Sequence,
    slice_or_i: slice | int,
    need: frozenset = frozenset(),
    ignore: frozenset = frozenset(),
    error_handling: bool = False,
    v: Sequence = (),
    check_args: bool = True,
    check_args_transform: bool = True,
) -> Sequence:
    """
    Returns a Sequence (matrix) containing choices elements

    Args:
        Sequence (obj): Sequence
        slice | int (slice_or_i): slice or index
        bool (error_handling): if True, raises an exception for invalid index
        int (v): default value for invalid index
        bool (check_args_type): check if type of arguments is valid
    Returns:
        Sequence: the collection
    """
    if check_args:
        if check_args_transform:
            if not isinstance(ignore, frozenset) and isinstance(ignore, str):
                ignore = frozenset({ignore})
            elif not isinstance(ignore, frozenset):
                ignore = frozenset(ignore)
            if not need:
                need = frozenset(range(len(obj)))
            elif not isinstance(need, frozenset) and isinstance(need, str):
                need = frozenset({need})
            elif not isinstance(need, frozenset):
                need = frozenset(need)

    if isinstance(obj, Iterable):
        return [
            (
                g_iloc_coll(
                    value,
                    slice_or_i,
                    error_handling=error_handling,
                    v=v
                ) if error_handling
                else value[slice_or_i]
            )
            for i, value in enumerate(obj)
            if i in need and i not in ignore
        ]
    return obj

def g_iloc_dict(
    obj: dict,
    slice_or_i: slice | int,
    need: frozenset = frozenset(),
    ignore: frozenset = frozenset(),
    error_handling: bool = False,
    v: Sequence = (),
    check_args: bool = True,
    check_args_transform: bool = True,
) -> dict:
    """
    Returns a dictionary with the selected keys

    Args:
        dict (obj): dictionary
        slice | int (slice_or_i): slice or index
        bool (error_handling): if True, raises an exception for invalid index
        dict (v): default value for invalid index
        bool (check_args_transform): check if transform arguments are valid
    Returns:
        dict: dictionary with the selected arguments
    """
    if check_args:
        if check_args_transform:
            if not isinstance(ignore, frozenset) and isinstance(ignore, str):
                ignore = frozenset({ignore})
            elif not isinstance(ignore, frozenset):
                ignore = frozenset(ignore)
            if not need:
                need = frozenset(obj.keys())
            elif not isinstance(need, frozenset) and isinstance(need, str):
                need = frozenset({need})
            elif not isinstance(need, frozenset):
                need = frozenset(need)

    if isinstance(obj, dict):
        return {
            key: g_iloc_coll(
                value,
                slice_or_i=slice_or_i,
                error_handling=error_handling,
                v=v,
            )
            for key, value in obj.items()
            if key in need and key not in ignore
        }
    return obj

def lstrip(
    str_: str,
    str_with: str = "_",
    step: int = 0,
    invert: bool = False,
) -> str:
    """
    returns a string cut off on the left side

    Args:
        str (str_): string to cut off
        str (str_with): up to what symbol
        int (step): step to start cutting
        bool (invert): if True, cuts from the right side
    Returns:
        str: cut off string
    """
    len_str_with = len(str_with)

    return next(
        (
            (
                str_[i + step:]
                if not invert
                else str_[:i - step]
            )
            for i in range(len(str_))
            if all((
                str_[i - len_str_with:i] == str_with,
                i >= len_str_with
            ))
        ),
        str_,
    )

def rstrip(
    str_: str,
    str_with: str = "_",
    step: int = 0,
    invert: bool = False,
) -> str:
    """
    returns a string cut off on the right side

    Args:
        str (str_): string to cut off
        str (str_with): up to what symbol
        int (step): step to start cutting
        bool (invert): if True, cuts from the left side
    Returns:
        str: cut off string
    """
    len_str_with = len(str_with)
    str_with_invert = str_with[::-1]
    str_invert = str_[::-1]

    return next(
        (
            (
                str_invert[i + step:]
                if not invert
                else str_invert[:i - step]
            )[::-1]
            for i in range(len(str_))
            if all((
                i >= len_str_with,
                str_invert[i - len_str_with:i] == str_with_invert
            ))
        ),
        str_,
    )

def g_roll_replace_array(
    arr: np.ndarray,
    shift: int,
    fill_value: Any,
    axis: None | int = None,
    check_args_transform: bool = True,
) -> np.ndarray:
    """
    returns roll replacement array

    Args:
        shift: number of shift
        fill_value: value to fill
        axis: axis to shift
        check_args_transform: check if transform arguments are valid
    Returns:
        np.ndarray: rolled replacement array
    """
    if check_args_transform:
        if not isinstance(arr, np.ndarray):
            arr = np.array(arr)

    arr = np.roll(arr, shift, axis=axis)
    shift_abs = np.abs(shift)

    if shift_abs > 0:
        if -2 < shift < 1:
            arr[shift] = fill_value
        else:
            if shift > 0:
                arr[:shift + 1] = fill_value
            else:
                arr[shift:] = fill_value
    return arr

def g_replace(
    arr: np.ndarray,
    slice_or_index: slice | int,
    fill_value: Any,
    use_copy: bool = False,
    check_args_transf: bool = True,
) -> np.ndarray:
    """
    returns a array with raplaced values

    Args:
        np.ndarray (arr): the array to be modified
        slice | int (slice_or_index): slice or index to replace
        fill_value: value to replace
        bool (use_copy): if True, creates a copy of the array
        bool (check_args_transf): check if transform arguments are valid
    Returns:
        np.ndarray: the array with the replaced values
    """
    if check_args_transf:
        if not isinstance(arr, np.ndarray):
            arr = np.array(arr)

    if use_copy:
        arr = arr.copy()
    arr[slice_or_index] = fill_value
    return arr

def wrap(
    func: Callable,
    exc_value: Any = 0,
    check_args_type: bool = True,
) -> Any:
    """
    in case of successful call returns the result of the
    called func. in case of any error returns the
    specified value

    Args:
        Callable (func): func to call
        Any (exc_value): value to return in case of error
        bool (check_args_type): check if type of arguments is valid
    Returns:
        Any: the value from the called func or the
            specified value
    """

    if check_args_type:
        assert callable(func), (
            f"type(func) ({type(func)}) != callable (func)"
        )

    try:
        return func()
    except:
        return exc_value

async def wrap_async(
    func: Coroutine,
    exc_value: Any = 0,
    check_args_type: bool = True,
) -> Any:
    """
    in case of successful call returns the result of the
    called func. in case of any error returns the
    specified value

    Args:
        Coroutine (func): func to call
        Any (exc_value): value to return in case of error
        bool (check_args_type): check if type of arguments is valid
    Returns:
        Any: the value from the called func or the
            specified value
    """

    if check_args_type:
        assert isinstance(func, Coroutine), (
            f"type(func) ({type(func)}) != Coroutine"
        )

    try:
        return await func
    except:
        return exc_value

def g_list_dicts_from_dict_arrays(
    dct: dict,
    check_args_type: bool = True,
) -> list:
    """
    moves arrays inside a dictionary to axis 0

    Args:
        dict (dct): dictionary with arrays
        bool (check_args_type): check if type of arguments is valid
    Returns:
        list: list of dicts
    """
    if check_args_type:
        assert isinstance(dct, dict), (
            f"type(dct) ({type(dct)}) != dict"
        )

    keys = dct.keys()

    return [
        dict(zip(keys, v))
        for v in zip(*dct.values())
    ]

def s_print(
    *args: int | float,
    precision: int = 4,
    **kwargs: Any,
) -> None:
    """
    outputs all arguments rounded to the specified value

    Args:
        tuple (args): tuple of numbers
        int (precision): precision to round numbers
        dict (kwargs): additional keyword arguments
    Returns:
        None: prints the rounded arguments
    """
    print(*[round(arg, precision) for arg in args], **kwargs,)

def g_drop_positive(
    arr: np.ndarray,
    check_args_transform: bool = True,
) -> np.ndarray:
    """
    returns array without positive values

    Args:
        np.ndarray (arr): array to drop positive values
    Returns:
        np.ndarray: array with negative values
    """
    if check_args_transform:
        if not isinstance(arr, np.ndarray):
            arr = np.array(arr)

    return arr[arr < 0]

def g_drop_negative(
    arr: np.ndarray,
    check_args_transform: bool = True,
) -> np.ndarray:
    """
    returns array without negative values

    Args:
        np.ndarray (arr): array to drop negative values
    Returns:
        np.ndarray: array with positive values
    """
    if check_args_transform:
        if not isinstance(arr, np.ndarray):
            arr = np.array(arr)

    return arr[arr > 0]
