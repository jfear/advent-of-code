from io import StringIO
import pandas as pd


def part2(input: str) -> str:
    df = pd.read_csv(StringIO(input), sep=r"\s+", header=None, names=["col1", "col2"])
    left = df["col1"].to_frame()
    right = df["col2"].value_counts().rename_axis("col1").rename("cnt").reset_index()
    value = left.merge(right).assign(product=lambda x: x.col1 * x.cnt)["product"].sum()
    return f"{value}"
