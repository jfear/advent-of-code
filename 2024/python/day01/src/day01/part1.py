from io import StringIO
import pandas as pd


def part1(input: str) -> str:
    df = pd.read_csv(StringIO(input), sep=r"\s+", header=None, names=["col1", "col2"])
    left = df.col1.sort_values().reset_index(drop=True)
    right = df.col2.sort_values().reset_index(drop=True)
    value = (left - right).abs().sum()
    return f"{value}"
