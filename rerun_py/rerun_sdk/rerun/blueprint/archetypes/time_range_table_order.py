# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/blueprint/archetypes/time_range_table_order.fbs".

# You can extend this class by creating a "TimeRangeTableOrderExt" class in "time_range_table_order_ext.py".

from __future__ import annotations

from typing import Any

from attrs import define, field

from ..._baseclasses import (
    Archetype,
)
from ...blueprint import components as blueprint_components
from ...error_utils import catch_and_log_exceptions

__all__ = ["TimeRangeTableOrder"]


@define(str=False, repr=False, init=False)
class TimeRangeTableOrder(Archetype):
    """**Archetype**: Ordering of the time range table of the dataframe view."""

    def __init__(
        self: Any,
        *,
        sort_key: blueprint_components.SortKeyLike | None = None,
        sort_order: blueprint_components.SortOrderLike | None = None,
    ):
        """
        Create a new instance of the TimeRangeTableOrder archetype.

        Parameters
        ----------
        sort_key:
            The primary sort key
        sort_order:
            The sort order

        """

        # You can define your own __init__ function as a member of TimeRangeTableOrderExt in time_range_table_order_ext.py
        with catch_and_log_exceptions(context=self.__class__.__name__):
            self.__attrs_init__(sort_key=sort_key, sort_order=sort_order)
            return
        self.__attrs_clear__()

    def __attrs_clear__(self) -> None:
        """Convenience method for calling `__attrs_init__` with all `None`s."""
        self.__attrs_init__(
            sort_key=None,  # type: ignore[arg-type]
            sort_order=None,  # type: ignore[arg-type]
        )

    @classmethod
    def _clear(cls) -> TimeRangeTableOrder:
        """Produce an empty TimeRangeTableOrder, bypassing `__init__`."""
        inst = cls.__new__(cls)
        inst.__attrs_clear__()
        return inst

    sort_key: blueprint_components.SortKeyBatch | None = field(
        metadata={"component": "optional"},
        default=None,
        converter=blueprint_components.SortKeyBatch._optional,  # type: ignore[misc]
    )
    # The primary sort key
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    sort_order: blueprint_components.SortOrderBatch | None = field(
        metadata={"component": "optional"},
        default=None,
        converter=blueprint_components.SortOrderBatch._optional,  # type: ignore[misc]
    )
    # The sort order
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    __str__ = Archetype.__str__
    __repr__ = Archetype.__repr__  # type: ignore[assignment]