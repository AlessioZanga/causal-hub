import pandas as pd
from causal_hub.datasets import CatTrj, CatTrjs, CatTrjEv, CatTrjsEv


def test_categorical_trajectory():
    # Create a sample DataFrame with a time column and categorical columns.
    df = pd.DataFrame(
        {
            "time": [0, 1, 2, 3, 4],
            "column_1": ["A", "B", "A", "C", "B"],
            "column_2": ["X", "Y", "X", "Z", "Y"]
        }
    )

    # Set data type for time column.
    df["time"] = df["time"].astype("float64")
    # Set data types for categorical columns.
    columns = list(set(df.columns) - {"time"})
    df[columns] = df[columns].astype("category")
    # Create a CatTrj object.
    trj = CatTrj(df)

    # Check the variables.
    assert trj.labels() == ["column_1", "column_2"], "Wrong labels."
    # Check the states of the variables.
    assert trj.states()["column_1"] == ("A", "B", "C"), "Wrong states."
    assert trj.states()["column_2"] == ("X", "Y", "Z"), "Wrong states."


def test_categorical_trajectory_with_states():
    # Create a sample DataFrame with a time column and categorical columns.
    df = pd.DataFrame(
        {
            "time": [0, 1, 2, 3, 4],
            "column_1": ["A", "B", "A", "C", "B"],
            "column_2": ["X", "Y", "X", "Z", "Y"]
        }
    )
    # Define some unobserved states.
    states = {
        "column_1": ("A", "B", "C", "D"),
        "column_2": ("X", "Y", "Z", "W")
    }

    # Set data type for time column.
    df["time"] = df["time"].astype("float64")
    # Set data types for categorical columns.
    columns = list(set(df.columns) - {"time"})
    df[columns] = df[columns].astype("category")
    # Create a CatTrj object.
    trj = CatTrj(df, with_states=states)

    # Check the variables.
    assert trj.labels() == ["column_1", "column_2"], "Wrong labels."
    # Check the states of the variables.
    assert trj.states()["column_1"] == ("A", "B", "C", "D"), "Wrong states."
    assert trj.states()["column_2"] == ("W", "X", "Y", "Z"), "Wrong states."


def test_categorical_trajectories():
    # Create tww sample DataFrames with a time column and categorical columns.
    dfs = [pd.DataFrame(
        {
            "time": [0, 1, 2, 3, 4],
            "column_1": ["A", "B", "A", "C", "B"],
            "column_2": ["X", "Y", "X", "Z", "Y"]
        }
    ),
        pd.DataFrame(
        {
            "time": [0, 1, 2, 3, 4],
            "column_1": ["A", "B", "A", "C", "B"],
            "column_2": ["X", "Y", "X", "Z", "Y"]
        }
    )]

    # For each dataframe ...
    for df in dfs:
        # Set data type for time column.
        df["time"] = df["time"].astype("float64")
        # Set data types for categorical columns.
        columns = list(set(df.columns) - {"time"})
        df[columns] = df[columns].astype("category")

    # Create a CatTrjs object.
    trjs = CatTrjs(dfs)

    # Check the variables.
    assert trjs.labels() == ["column_1", "column_2"], "Wrong labels."
    # Check the states of the variables.
    assert trjs.states()["column_1"] == ("A", "B", "C"), "Wrong states."
    assert trjs.states()["column_2"] == ("X", "Y", "Z"), "Wrong states."


def test_categorical_trajectories_with_states():
    # Create tww sample DataFrames with a time column and categorical columns.
    dfs = [pd.DataFrame(
        {
            "time": [0, 1, 2, 3, 4],
            "column_1": ["A", "B", "A", "C", "B"],
            "column_2": ["X", "Y", "X", "Z", "Y"]
        }
    ),
        pd.DataFrame(
        {
            "time": [0, 1, 2, 3, 4],
            "column_1": ["A", "B", "A", "C", "B"],
            "column_2": ["X", "Y", "X", "Z", "Y"]
        }
    )]
    # Define some unobserved states.
    states = {
        "column_1": ("A", "B", "C", "D"),
        "column_2": ("X", "Y", "Z", "W")
    }

    # For each dataframe ...
    for df in dfs:
        # Set data type for time column.
        df["time"] = df["time"].astype("float64")
        # Set data types for categorical columns.
        columns = list(set(df.columns) - {"time"})
        df[columns] = df[columns].astype("category")

    # Create a CatTrjs object.
    trjs = CatTrjs(dfs, with_states=states)

    # Check the variables.
    assert trjs.labels() == ["column_1", "column_2"], "Wrong labels."
    # Check the states of the variables.
    assert trjs.states()["column_1"] == ("A", "B", "C", "D"), "Wrong states."
    assert trjs.states()["column_2"] == ("W", "X", "Y", "Z"), "Wrong states."


def test_categorical_trajectory_evidence():
    # Create a sample DataFrame with `event`, `state`, `start_time`, and `end_time` columns.
    df = pd.DataFrame(
        {
            "event": ["A", "B", "A", "C", "B"],
            "state": ["X", "Y", "X", "Z", "Y"],
            "start_time": [0, 1, 2, 3, 4],
            "end_time": [1, 2, 3, 4, 5]
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
    trj_ev = CatTrjEv(df, with_states=states)

    # Check the variables.
    assert trj_ev.labels() == ["A", "B", "C"], "Wrong labels."
    # Check the states of the variables.
    assert trj_ev.states()["A"] == ("X", "Y", "Z"), "Wrong states."
    assert trj_ev.states()["B"] == ("X", "Y", "Z"), "Wrong states."
    assert trj_ev.states()["C"] == ("Y", "Z"), "Wrong states."
