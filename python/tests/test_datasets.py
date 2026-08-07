import numpy as np
import pandas as pd
import pytest
from causal_hub import Error
from causal_hub.datasets import (
    CatEv,
    CatIncTable,
    CatTable,
    CatTrj,
    CatTrjEv,
    CatTrjs,
    CatTrjsEv,
    GaussEv,
    GaussIncTable,
    GaussTable,
    MissingMechanism,
    MissingTable,
)


def test_categorical_table() -> None:
    """Test creation and conversion of complete Categorical Table."""
    # Create a sample DataFrame with categorical columns.
    df = pd.DataFrame(
        {
            "column_1": ["A", "B", "A", "C", "B"],
            "column_2": ["X", "Y", "X", "Z", "Y"],
        }
    )

    # Set data types for categorical columns.
    df = df.astype("category")
    # Create a CatTable object.
    table = CatTable.from_pandas(df)

    # Check the variables.
    assert table.labels() == ["column_1", "column_2"], "Wrong labels."
    # Check the states of the variables.
    assert table.support()["column_1"] == ("A", "B", "C"), "Wrong states."
    assert table.support()["column_2"] == ("X", "Y", "Z"), "Wrong states."
    # Check the values of the variables.
    np.testing.assert_array_equal(
        table.values(),
        np.array(
            [
                [0, 0],
                [1, 1],
                [0, 0],
                [2, 2],
                [1, 1],
            ]
        ),
        "Wrong values.",
    )
    # Convert back to pandas DataFrame and check equality.
    pd.testing.assert_frame_equal(df, table.to_pandas())


def test_gaussian_table() -> None:
    """Test creation and conversion of complete Gaussian Table."""
    # Create a sample DataFrame with float64 columns.
    df = pd.DataFrame(
        {
            "column_1": [1.0, 2.0, 3.0, 4.0, 5.0],
            "column_2": [5.0, 4.0, 3.0, 2.0, 1.0],
        }
    )
    # Set data types for float64 columns.
    df = df.astype("float64")
    # Create a GaussTable object.
    table = GaussTable.from_pandas(df)

    # Check the variables.
    assert table.labels() == ["column_1", "column_2"], "Wrong labels."
    # Check the values of the variables.
    np.testing.assert_array_equal(
        table.values(),
        np.array(
            [
                [1.0, 5.0],
                [2.0, 4.0],
                [3.0, 3.0],
                [4.0, 2.0],
                [5.0, 1.0],
            ]
        ),
        "Wrong values.",
    )
    # Convert back to pandas DataFrame and check equality.
    pd.testing.assert_frame_equal(df, table.to_pandas())


def test_categorical_evidence_from_dict() -> None:
    """Test creation of categorical evidence from a dictionary."""
    ev = CatEv.from_dict(
        {"A": "a1", "C": "c0"},
        with_states={"A": ["a0", "a1"], "B": ["b0", "b1"], "C": ["c0", "c1"]},
    )

    assert ev.labels() == ["A", "B", "C"], "Wrong evidence labels."
    assert ev.support()["A"] == ("a0", "a1"), "Wrong A states."
    assert ev.support()["B"] == ("b0", "b1"), "Wrong B states."
    assert ev.support()["C"] == ("c0", "c1"), "Wrong C states."


def test_gaussian_evidence_from_dict() -> None:
    """Test creation of gaussian evidence from a dictionary."""
    ev = GaussEv.from_dict({"X": 1.0, "Z": -2.0}, with_labels=["X", "Y", "Z"])

    assert ev.labels() == ["X", "Y", "Z"], "Wrong evidence labels."


def test_categorical_trajectory() -> None:
    """Test creation and conversion of Categorical Trajectory (single)."""
    # Create a sample DataFrame with a time column and categorical columns.
    df = pd.DataFrame(
        {
            "time": [0, 1, 2, 3, 4],
            "column_1": ["A", "A", "B", "C", "C"],
            "column_2": ["X", "Y", "Y", "Y", "Z"],
        }
    )

    # Set data type for time column.
    df["time"] = df["time"].astype("float64")
    # Set data types for categorical columns.
    columns = list(set(df.columns) - {"time"})
    df[columns] = df[columns].astype("category")
    # Create a CatTrj object.
    trj = CatTrj.from_pandas(df)

    # Check the variables.
    assert trj.labels() == ["column_1", "column_2"], "Wrong labels."
    # Check the states of the variables.
    assert trj.support()["column_1"] == ("A", "B", "C"), "Wrong states."
    assert trj.support()["column_2"] == ("X", "Y", "Z"), "Wrong states."
    # Check the time values.
    np.testing.assert_array_equal(
        trj.times(),
        np.array([0.0, 1.0, 2.0, 3.0, 4.0]),
        "Wrong time.",
    )
    # Check the values of the variables.
    np.testing.assert_array_equal(
        trj.values(),
        np.array(
            [
                [0, 0],
                [0, 1],
                [1, 1],
                [2, 1],
                [2, 2],
            ]
        ),
        "Wrong values.",
    )
    # Convert back to pandas DataFrame and check equality.
    pd.testing.assert_frame_equal(df, trj.to_pandas())


def test_categorical_trajectory_with_states() -> None:
    """Test creation of Categorical Trajectory with predefined states."""
    # Create a sample DataFrame with a time column and categorical columns.
    df = pd.DataFrame(
        {
            "time": [0, 1, 2, 3, 4],
            "column_1": ["A", "A", "B", "C", "C"],
            "column_2": ["X", "Y", "Y", "Y", "Z"],
        }
    )

    # Define some unobserved states.
    states = {
        "column_1": ("A", "B", "C", "D"),
        "column_2": ("X", "Y", "Z", "W"),
    }

    # Set data type for time column.
    df["time"] = df["time"].astype("float64")
    # Set data types for categorical columns.
    columns = list(set(df.columns) - {"time"})
    df[columns] = df[columns].astype("category")
    # Add the unobserved states to the dataframe categories.
    df["column_1"] = df["column_1"].cat.set_categories(states["column_1"])
    df["column_2"] = df["column_2"].cat.set_categories(states["column_2"])

    # Create a CatTrj object.
    trj = CatTrj.from_pandas(df)

    # Check the variables.
    assert trj.labels() == ["column_1", "column_2"], "Wrong labels."
    # Check the states of the variables.
    assert trj.support()["column_1"] == ("A", "B", "C", "D"), "Wrong states."
    assert trj.support()["column_2"] == ("W", "X", "Y", "Z"), "Wrong states."
    # Check time values.
    np.testing.assert_array_equal(trj.times(), np.array([0.0, 1.0, 2.0, 3.0, 4.0]))
    # Check encoded values w.r.t. provided category order.
    np.testing.assert_array_equal(
        trj.values(),
        np.array(
            [
                [0, 1],
                [0, 2],
                [1, 2],
                [2, 2],
                [2, 3],
            ]
        ),
    )
    # Check round-trip values (ignore categorical ordering differences).
    out = trj.to_pandas()
    pd.testing.assert_series_equal(df["time"], out["time"])
    pd.testing.assert_series_equal(
        df["column_1"].astype("string"),
        out["column_1"].astype("string"),
        check_names=False,
    )
    pd.testing.assert_series_equal(
        df["column_2"].astype("string"),
        out["column_2"].astype("string"),
        check_names=False,
    )


def test_categorical_trajectories() -> None:
    """Test creation and conversion of multiple Categorical Trajectories."""
    # Create two sample DataFrames with a time column and categorical columns.
    dfs = [
        pd.DataFrame(
            {
                "time": [0, 1, 2, 3, 4],
                "column_1": ["A", "A", "B", "C", "C"],
                "column_2": ["X", "Y", "Y", "Y", "Z"],
            }
        ),
        pd.DataFrame(
            {
                "time": [0, 1, 2, 3, 4],
                "column_1": ["A", "A", "B", "C", "C"],
                "column_2": ["X", "Y", "Y", "Y", "Z"],
            }
        ),
    ]

    # For each dataframe ...
    for df in dfs:
        # Set data type for time column.
        df["time"] = df["time"].astype("float64")
        # Set data types for categorical columns.
        columns = list(set(df.columns) - {"time"})
        df[columns] = df[columns].astype("category")

    # Create a CatTrjs object.
    trjs = CatTrjs.from_pandas(dfs)

    # Check the variables.
    assert trjs.labels() == ["column_1", "column_2"], "Wrong labels."
    # Check the states of the variables.
    assert trjs.support()["column_1"] == ("A", "B", "C"), "Wrong states."
    assert trjs.support()["column_2"] == ("X", "Y", "Z"), "Wrong states."
    # Check the number of trajectories.
    assert len(trjs.values()) == 2, "Wrong number of trajectories."
    # Check the time values of the first trajectory.
    np.testing.assert_array_equal(
        trjs.values()[0].times(),
        np.array([0.0, 1.0, 2.0, 3.0, 4.0]),
        "Wrong time.",
    )
    # Check the values of the first trajectory.
    np.testing.assert_array_equal(
        trjs.values()[0].values(),
        np.array(
            [
                [0, 0],
                [0, 1],
                [1, 1],
                [2, 1],
                [2, 2],
            ]
        ),
        "Wrong values.",
    )
    # Check the time values of the second trajectory.
    np.testing.assert_array_equal(
        trjs.values()[1].times(),
        np.array([0.0, 1.0, 2.0, 3.0, 4.0]),
        "Wrong time.",
    )
    # Check the values of the second trajectory.
    np.testing.assert_array_equal(
        trjs.values()[1].values(),
        np.array(
            [
                [0, 0],
                [0, 1],
                [1, 1],
                [2, 1],
                [2, 2],
            ]
        ),
        "Wrong values.",
    )
    # Convert back to list of pandas DataFrames and check equality.
    for df, trj in zip(dfs, trjs.to_pandas()):
        pd.testing.assert_frame_equal(df, trj)


def test_categorical_trajectories_with_states() -> None:
    """Test creation of multiple Categorical Trajectories with predefined states."""
    # Create two sample DataFrames with a time column and categorical columns.
    dfs = [
        pd.DataFrame(
            {
                "time": [0, 1, 2, 3, 4],
                "column_1": ["A", "A", "B", "C", "C"],
                "column_2": ["X", "Y", "Y", "Y", "Z"],
            }
        ),
        pd.DataFrame(
            {
                "time": [0, 1, 2, 3, 4],
                "column_1": ["A", "A", "B", "C", "C"],
                "column_2": ["X", "Y", "Y", "Y", "Z"],
            }
        ),
    ]

    # Define some unobserved states.
    states = {
        "column_1": ("A", "B", "C", "D"),
        "column_2": ("X", "Y", "Z", "W"),
    }

    # For each dataframe ...
    for df in dfs:
        # Set data type for time column.
        df["time"] = df["time"].astype("float64")
        # Set data types for categorical columns.
        columns = list(set(df.columns) - {"time"})
        df[columns] = df[columns].astype("category")
        # Add the unobserved states to the dataframe categories.
        df["column_1"] = df["column_1"].cat.set_categories(states["column_1"])
        df["column_2"] = df["column_2"].cat.set_categories(states["column_2"])

    # Create a CatTrjs object.
    trjs = CatTrjs.from_pandas(dfs)

    # Check the variables.
    assert trjs.labels() == ["column_1", "column_2"], "Wrong labels."
    # Check the states of the variables.
    assert trjs.support()["column_1"] == ("A", "B", "C", "D"), "Wrong states."
    assert trjs.support()["column_2"] == ("W", "X", "Y", "Z"), "Wrong states."
    # Check the number of trajectories.
    assert len(trjs.values()) == 2, "Wrong number of trajectories."
    # Check the time values of the first trajectory.
    np.testing.assert_array_equal(
        trjs.values()[0].times(),
        np.array([0.0, 1.0, 2.0, 3.0, 4.0]),
        "Wrong time.",
    )
    # Check the values of the first trajectory.
    np.testing.assert_array_equal(
        trjs.values()[0].values(),
        np.array(
            [
                [0, 1],
                [0, 2],
                [1, 2],
                [2, 2],
                [2, 3],
            ]
        ),
        "Wrong values",
    )
    # Check the time values of the second trajectory.
    np.testing.assert_array_equal(
        trjs.values()[1].times(), np.array([0.0, 1.0, 2.0, 3.0, 4.0]), "Wrong time."
    )
    # Check the values of the second trajectory.
    np.testing.assert_array_equal(
        trjs.values()[1].values(),
        np.array(
            [
                [0, 1],
                [0, 2],
                [1, 2],
                [2, 2],
                [2, 3],
            ]
        ),
        "Wrong values.",
    )
    # Convert back to list of pandas DataFrames and check equality.
    for df, trj in zip(dfs, trjs.to_pandas()):
        # Sort categories to ensure consistent ordering for comparison.
        df["column_1"] = df["column_1"].cat.set_categories(sorted(states["column_1"]))
        df["column_2"] = df["column_2"].cat.set_categories(sorted(states["column_2"]))
        pd.testing.assert_frame_equal(df, trj)


def test_categorical_trajectory_evidence() -> None:
    """Test creation of Categorical Trajectory Evidence handling."""
    # Create a sample DataFrame with `event`, `state`, `start_time`, and `end_time` columns.
    df = pd.DataFrame(
        {
            "event": ["A", "B", "A", "C", "B"],
            "state": ["X", "Y", "X", "Z", "Y"],
            "start_time": [0, 1, 2, 3, 4],
            "end_time": [1, 2, 3, 4, 5],
        }
    )

    # Define some unobserved states.
    states = {
        "B": ("X", "Y", "Z"),
        "C": ("Y", "Z"),
        "A": ("X", "Y", "Z"),
    }

    # Set data type for time columns.
    time_columns = ["start_time", "end_time"]
    df[time_columns] = df[time_columns].astype("float64")

    # Create a CatTrjEv object.
    trj_ev = CatTrjEv.from_pandas(df, with_states=states)

    # Check the variables.
    assert trj_ev.labels() == ["A", "B", "C"], "Wrong labels."
    # Check the states of the variables.
    assert trj_ev.support()["A"] == ("X", "Y", "Z"), "Wrong states."
    assert trj_ev.support()["B"] == ("X", "Y", "Z"), "Wrong states."
    assert trj_ev.support()["C"] == ("Y", "Z"), "Wrong states."

    # Also check inferred states path (without explicit states).
    trj_ev_inferred = CatTrjEv.from_pandas(df)
    assert trj_ev_inferred.labels() == ["A", "B", "C"], "Wrong inferred labels."
    assert trj_ev_inferred.support()["A"] == ("X",), "Wrong inferred A states."
    assert trj_ev_inferred.support()["B"] == ("Y",), "Wrong inferred B states."
    assert trj_ev_inferred.support()["C"] == ("Z",), "Wrong inferred C states."


def test_categorical_incomplete_table() -> None:
    """Test creation and conversion of Categorical Incomplete Table (with missing values)."""
    # Create a sample DataFrame with categorical columns and missing values.
    df = pd.DataFrame(
        {
            "column_1": ["A", "B", "A", np.nan, "B"],
            "column_2": ["X", "Y", np.nan, "Z", "Y"],
        }
    )

    # Set data types for categorical columns.
    df = df.astype("category")
    # Create a CatIncTable object.
    table = CatIncTable.from_pandas(df)

    # Check the variables.
    assert table.labels() == ["column_1", "column_2"], "Wrong labels."
    # Check the states of the variables.
    assert table.support()["column_1"] == ("A", "B"), "Wrong states."
    assert table.support()["column_2"] == ("X", "Y", "Z"), "Wrong states."

    # Check the missing information.
    missing = table.missing()
    assert isinstance(missing, MissingTable), "Wrong missing type."
    assert missing.labels() == ["column_1", "column_2"], "Wrong missing labels."
    assert missing.missing_count() == 2, "Wrong missing count."
    assert missing.missing_rate() == 0.2, "Wrong missing rate."
    np.testing.assert_array_equal(
        missing.missing_mask(),
        np.array(
            [
                [False, False],
                [False, False],
                [False, True],
                [True, False],
                [False, False],
            ]
        ),
        "Wrong missing mask.",
    )
    np.testing.assert_array_equal(
        missing.missing_count_by_cols(),
        np.array([1, 1], dtype=np.uint64),
        "Wrong missing count by cols.",
    )
    np.testing.assert_array_equal(
        missing.missing_count_by_rows(),
        np.array([0, 0, 1, 1, 0], dtype=np.uint64),
        "Wrong missing count by rows.",
    )

    # Check the values of the variables.
    # MISSING is CatType::MAX which is 255.
    np.testing.assert_array_equal(
        table.values(),
        np.array(
            [
                [0, 0],
                [1, 1],
                [0, 255],
                [255, 2],
                [1, 1],
            ]
        ),
        "Wrong values.",
    )

    # Convert back to pandas DataFrame and check equality.
    pd.testing.assert_frame_equal(df, table.to_pandas())


def test_missing_table_numerical() -> None:
    """Test MissingTable from numerical mask."""
    # Create a MissingTable.
    # labels, mask.
    mask = np.array(
        [[False, True], [True, False], [False, False]],
        dtype=bool,
    )
    labels = ["A", "B"]

    missing = MissingTable(labels, mask)

    assert missing.labels() == labels
    np.testing.assert_array_equal(missing.missing_mask(), mask)
    assert missing.missing_count() == 2
    assert missing.missing_rate() == 1.0 / 3.0
    np.testing.assert_array_equal(
        missing.missing_mask_by_cols(), np.array([1, 1], dtype=np.uint8)
    )
    np.testing.assert_array_equal(
        missing.missing_mask_by_rows(), np.array([1, 1, 0], dtype=np.uint8)
    )
    np.testing.assert_array_equal(
        missing.missing_count_by_cols(), np.array([1, 1], dtype=np.uint64)
    )
    np.testing.assert_array_equal(
        missing.missing_count_by_rows(), np.array([1, 1, 0], dtype=np.uint64)
    )


def test_gaussian_incomplete_table() -> None:
    """Test creation and conversion of Gaussian Incomplete Table (with missing values)."""
    # Create a sample DataFrame with missing values.
    df = pd.DataFrame(
        {
            "A": [0.0, 1.0, 2.0, np.nan, 0.0, 1.0, np.nan, np.nan],
            "B": [1.0, 0.0, 1.0, 0.0, np.nan, 1.0, np.nan, 1.0],
            "C": [2.0, 2.0, 0.0, 1.0, 3.0, np.nan, np.nan, 3.0],
        }
    )

    # Create a GaussIncTable object.
    table = GaussIncTable.from_pandas(df)

    # Check the variables.
    assert table.labels() == ["A", "B", "C"]

    # Check values.
    np.testing.assert_array_equal(table.values(), df.to_numpy())

    # Convert back.
    pd.testing.assert_frame_equal(df, table.to_pandas())

    # Check missing.
    missing = table.missing()
    assert missing.missing_count() == 7
    assert missing.missing_rate() == 7 / 24
    np.testing.assert_array_equal(
        missing.missing_count_by_cols(), np.array([3, 2, 2], dtype=np.uint64)
    )
    np.testing.assert_array_equal(
        missing.missing_count_by_rows(),
        np.array([0, 0, 0, 1, 1, 1, 3, 1], dtype=np.uint64),
    )


def test_missing_mechanism_creation() -> None:
    labels = ["X", "Y", "Z"]
    # Variable 0 (X) is missing due to variable 1 (Y) and 2 (Z)
    # Variable 1 (Y) is missing due to variable 2 (Z)
    pr = {0: {1, 2}, 1: {2}}

    mechanism = MissingMechanism(labels, pr)

    assert mechanism.labels() == labels
    assert len(mechanism) == 2
    assert not mechanism.is_empty()
    assert set(mechanism.keys()) == {0, 1}

    values = mechanism.values()
    assert len(values) == 2
    assert {1, 2} in values
    assert {2} in values

    assert mechanism.contains_key(0)
    assert mechanism.contains_key(1)
    assert not mechanism.contains_key(2)

    assert mechanism.get(0) == {1, 2}
    assert mechanism.get(1) == {2}
    assert mechanism.get(2) is None


def test_missing_mechanism_insert() -> None:
    labels = ["X", "Y", "Z"]
    pr = {0: {1}}
    mechanism = MissingMechanism(labels, pr)

    assert len(mechanism) == 1
    assert mechanism.get(0) == {1}

    mechanism.insert(1, {2})
    assert len(mechanism) == 2
    assert mechanism.get(1) == {2}

    mechanism.insert(0, {1, 2})
    assert mechanism.get(0) == {1, 2}


def test_missing_mechanism_error() -> None:
    labels = ["X", "Y"]
    # Index 2 is out of bounds
    pr = {0: {2}}
    with pytest.raises(Error):
        MissingMechanism(labels, pr)


def _require_polars():
    return pytest.importorskip("polars")


def test_categorical_table_polars() -> None:
    pl = _require_polars()

    df = pl.DataFrame(
        {
            "column_1": ["A", "B", "A", "C", "B"],
            "column_2": ["X", "Y", "X", "Z", "Y"],
        }
    ).with_columns(
        pl.col("column_1").cast(pl.Categorical),
        pl.col("column_2").cast(pl.Categorical),
    )

    table = CatTable.from_polars(df)
    assert table.labels() == ["column_1", "column_2"]
    assert {"A", "B", "C"}.issubset(set(table.support()["column_1"]))
    assert {"X", "Y", "Z"}.issubset(set(table.support()["column_2"]))

    values = table.values()
    assert values.shape == (5, 2)
    # Decode internal codes through states and verify original values.
    decoded_col1 = [table.support()["column_1"][int(x)] for x in values[:, 0]]
    decoded_col2 = [table.support()["column_2"][int(x)] for x in values[:, 1]]
    assert decoded_col1 == ["A", "B", "A", "C", "B"]
    assert decoded_col2 == ["X", "Y", "X", "Z", "Y"]

    out = table.to_polars()
    assert out.columns == ["column_1", "column_2"]
    assert out.shape == (5, 2)
    assert out["column_1"].to_list() == ["A", "B", "A", "C", "B"]
    assert out["column_2"].to_list() == ["X", "Y", "X", "Z", "Y"]


def test_gaussian_table_polars() -> None:
    pl = _require_polars()

    df = pl.DataFrame(
        {
            "column_1": [1.0, 2.0, 3.0, 4.0, 5.0],
            "column_2": [5.0, 4.0, 3.0, 2.0, 1.0],
        }
    )

    table = GaussTable.from_polars(df)
    assert table.labels() == ["column_1", "column_2"]
    np.testing.assert_array_equal(
        table.values(),
        np.array(
            [
                [1.0, 5.0],
                [2.0, 4.0],
                [3.0, 3.0],
                [4.0, 2.0],
                [5.0, 1.0],
            ]
        ),
    )

    out = table.to_polars()
    assert out.columns == ["column_1", "column_2"]
    assert out.shape == (5, 2)
    assert out["column_1"].to_list() == [1.0, 2.0, 3.0, 4.0, 5.0]
    assert out["column_2"].to_list() == [5.0, 4.0, 3.0, 2.0, 1.0]


def test_categorical_incomplete_table_polars() -> None:
    pl = _require_polars()

    df = pl.DataFrame(
        {
            "column_1": ["A", "B", "A", None, "B"],
            "column_2": ["X", "Y", None, "Z", "Y"],
        }
    ).with_columns(
        pl.col("column_1").cast(pl.Categorical),
        pl.col("column_2").cast(pl.Categorical),
    )

    table = CatIncTable.from_polars(df)
    assert table.labels() == ["column_1", "column_2"]
    assert {"A", "B"}.issubset(set(table.support()["column_1"]))
    assert {"X", "Y", "Z"}.issubset(set(table.support()["column_2"]))

    values = table.values()
    assert values.shape == (5, 2)
    # Missing is encoded as 255.
    expected_missing = np.array(
        [
            [False, False],
            [False, False],
            [False, True],
            [True, False],
            [False, False],
        ]
    )
    np.testing.assert_array_equal(values == 255, expected_missing)

    missing = table.missing()
    assert missing.missing_count() == 2
    assert missing.missing_rate() == 0.2

    # Decode non-missing values through states and verify round-trip semantics.
    decoded_col1 = [
        None if int(x) == 255 else table.support()["column_1"][int(x)]
        for x in values[:, 0]
    ]
    decoded_col2 = [
        None if int(x) == 255 else table.support()["column_2"][int(x)]
        for x in values[:, 1]
    ]
    assert decoded_col1 == ["A", "B", "A", None, "B"]
    assert decoded_col2 == ["X", "Y", None, "Z", "Y"]

    out = table.to_polars()
    assert out.shape == (5, 2)
    assert out["column_1"].to_list() == ["A", "B", "A", None, "B"]
    assert out["column_2"].to_list() == ["X", "Y", None, "Z", "Y"]


def test_gaussian_incomplete_table_polars() -> None:
    pl = _require_polars()

    df = pl.DataFrame(
        {
            "A": [0.0, 1.0, 2.0, None],
            "B": [1.0, 0.0, None, 0.0],
            "C": [2.0, None, 0.0, 1.0],
        }
    ).with_columns(
        pl.col("A").cast(pl.Float64),
        pl.col("B").cast(pl.Float64),
        pl.col("C").cast(pl.Float64),
    )

    table = GaussIncTable.from_polars(df)
    assert table.labels() == ["A", "B", "C"]

    values = table.values()
    assert values.shape == (4, 3)
    expected = np.array(
        [
            [0.0, 1.0, 2.0],
            [1.0, 0.0, np.nan],
            [2.0, np.nan, 0.0],
            [np.nan, 0.0, 1.0],
        ]
    )
    np.testing.assert_allclose(values, expected, equal_nan=True)

    missing = table.missing()
    assert missing.missing_count() == 3
    assert missing.missing_rate() == 0.25

    out = table.to_polars()
    assert out.columns == ["A", "B", "C"]
    assert out.height == 4
    out_a = out["A"].to_list()
    out_b = out["B"].to_list()
    out_c = out["C"].to_list()
    assert out_a[:3] == [0.0, 1.0, 2.0]
    assert out_b[0] == 1.0 and out_b[1] == 0.0 and out_b[3] == 0.0
    assert out_c[0] == 2.0 and out_c[2] == 0.0 and out_c[3] == 1.0
    assert np.isnan(out_a[3])
    assert np.isnan(out_b[2])
    assert np.isnan(out_c[1])


def test_categorical_trajectory_polars() -> None:
    pl = _require_polars()

    df = pl.DataFrame(
        {
            "time": [0.0, 1.0, 2.0, 3.0, 4.0],
            "column_1": ["A", "A", "B", "C", "C"],
            "column_2": ["X", "Y", "Y", "Y", "Z"],
        }
    ).with_columns(
        pl.col("column_1").cast(pl.Categorical),
        pl.col("column_2").cast(pl.Categorical),
    )

    trj = CatTrj.from_polars(df)
    assert trj.labels() == ["column_1", "column_2"]
    np.testing.assert_array_equal(trj.times(), np.array([0.0, 1.0, 2.0, 3.0, 4.0]))

    values = trj.values()
    assert values.shape == (5, 2)
    decoded_col1 = [trj.support()["column_1"][int(x)] for x in values[:, 0]]
    decoded_col2 = [trj.support()["column_2"][int(x)] for x in values[:, 1]]
    assert decoded_col1 == ["A", "A", "B", "C", "C"]
    assert decoded_col2 == ["X", "Y", "Y", "Y", "Z"]

    out = trj.to_polars()
    assert out.columns == ["time", "column_1", "column_2"]
    assert out.shape == (5, 3)
    assert out["time"].to_list() == [0.0, 1.0, 2.0, 3.0, 4.0]
    assert out["column_1"].to_list() == ["A", "A", "B", "C", "C"]
    assert out["column_2"].to_list() == ["X", "Y", "Y", "Y", "Z"]


def test_categorical_trajectories_polars() -> None:
    pl = _require_polars()

    df1 = pl.DataFrame(
        {
            "time": [0.0, 1.0, 2.0],
            "A": ["a0", "a1", "a1"],
            "B": ["b0", "b0", "b1"],
        }
    ).with_columns(pl.col("A").cast(pl.Categorical), pl.col("B").cast(pl.Categorical))
    df2 = pl.DataFrame(
        {
            "time": [0.0, 1.0, 2.0],
            "A": ["a1", "a1", "a0"],
            "B": ["b1", "b0", "b0"],
        }
    ).with_columns(pl.col("A").cast(pl.Categorical), pl.col("B").cast(pl.Categorical))

    trjs = CatTrjs.from_polars([df1, df2])
    assert trjs.labels() == ["A", "B"]

    values = trjs.values()
    assert len(values) == 2
    np.testing.assert_array_equal(values[0].times(), np.array([0.0, 1.0, 2.0]))
    np.testing.assert_array_equal(values[1].times(), np.array([0.0, 1.0, 2.0]))

    v0 = values[0].values()
    v1 = values[1].values()
    dec0_a = [values[0].support()["A"][int(x)] for x in v0[:, 0]]
    dec0_b = [values[0].support()["B"][int(x)] for x in v0[:, 1]]
    dec1_a = [values[1].support()["A"][int(x)] for x in v1[:, 0]]
    dec1_b = [values[1].support()["B"][int(x)] for x in v1[:, 1]]
    assert dec0_a == ["a0", "a1", "a1"]
    assert dec0_b == ["b0", "b0", "b1"]
    assert dec1_a == ["a1", "a1", "a0"]
    assert dec1_b == ["b1", "b0", "b0"]

    out = trjs.to_polars()
    assert len(out) == 2
    assert out[0].columns == ["time", "A", "B"]
    assert out[0]["time"].to_list() == [0.0, 1.0, 2.0]
    assert out[0]["A"].to_list() == ["a0", "a1", "a1"]
    assert out[0]["B"].to_list() == ["b0", "b0", "b1"]
    assert out[1]["time"].to_list() == [0.0, 1.0, 2.0]
    assert out[1]["A"].to_list() == ["a1", "a1", "a0"]
    assert out[1]["B"].to_list() == ["b1", "b0", "b0"]


def test_categorical_trajectory_evidence_polars() -> None:
    pl = _require_polars()

    df = pl.DataFrame(
        {
            "event": ["A", "B", "A", "C", "B"],
            "state": ["X", "Y", "X", "Z", "Y"],
            "start_time": [0.0, 1.0, 2.0, 3.0, 4.0],
            "end_time": [1.0, 2.0, 3.0, 4.0, 5.0],
        }
    )

    states = {
        "B": ("X", "Y", "Z"),
        "C": ("Y", "Z"),
        "A": ("X", "Y", "Z"),
    }
    trj_ev = CatTrjEv.from_polars(df, with_states=states)
    assert trj_ev.labels() == ["A", "B", "C"]
    assert trj_ev.support()["A"] == ("X", "Y", "Z")
    assert trj_ev.support()["B"] == ("X", "Y", "Z")
    assert trj_ev.support()["C"] == ("Y", "Z")


def test_categorical_trajectories_evidence_polars() -> None:
    pl = _require_polars()

    df1 = pl.DataFrame(
        {
            "event": ["A", "B"],
            "state": ["X", "Y"],
            "start_time": [0.0, 1.0],
            "end_time": [1.0, 2.0],
        }
    )
    df2 = pl.DataFrame(
        {
            "event": ["C", "A"],
            "state": ["Z", "X"],
            "start_time": [0.0, 2.0],
            "end_time": [1.0, 3.0],
        }
    )

    with_states = {
        "A": ("X", "Y", "Z"),
        "B": ("X", "Y", "Z"),
        "C": ("X", "Y", "Z"),
    }
    trjs_ev = CatTrjsEv.from_polars([df1, df2], with_states=with_states)
    assert set(trjs_ev.labels()) == {"A", "B", "C"}
    assert trjs_ev.support()["A"] == ("X", "Y", "Z")
    assert trjs_ev.support()["B"] == ("X", "Y", "Z")
    assert trjs_ev.support()["C"] == ("X", "Y", "Z")
