import pandas as pd
import matplotlib.pyplot as plt
from tkinter import Tk, filedialog
# import seaborn as sns

def select_file():
    """Open a file dialog to select a file."""
    root = Tk()
    root.withdraw()  # Hide the root window
    file_path = filedialog.askopenfilename(
        title="Select a Text File",
        filetypes=[("Text Files", "*.txt"), ("CSV Files", "*.csv"), ("All Files", "*.*")]
    )
    return file_path

def scatter_time_by_mord(df):
    """Scatter Plots of Movement time, arranged by Move Ordering"""
    mord_0_alg_0_attacker = df["Average Attacker"]

def plot_game_length_distribution_including_n(df):
    """Plot the game length distribution including rows with Victory = 'N'."""
    length_counts = df["Length"].value_counts().sort_index()
    plt.figure(figsize=(12, 6))
    plt.bar(length_counts.index, length_counts.values, color='skyblue', edgecolor='black')
    plt.title('Game Length Distribution (Including Victory "N")')
    plt.xlabel('Game Length (Number of Moves)')
    plt.ylabel('Frequency')
    plt.xticks(range(0, df["Length"].max() + 1, 20))
    plt.grid(axis='y', linestyle='--', alpha=0.7)
    plt.tight_layout()
    plt.show()

def plot_game_length_distribution_excluding_n(df):
    """Plot the game length distribution excluding rows with Victory = 'N'."""
    df_excl_n = df[df["Victory"] != "N"]
    length_counts_excl_n = df_excl_n["Length"].value_counts().sort_index()
    plt.figure(figsize=(12, 6))
    plt.bar(length_counts_excl_n.index, length_counts_excl_n.values, color='skyblue', edgecolor='black')
    plt.title('Game Length Distribution (Excluding Victory "N")')
    plt.xlabel('Game Length (Number of Moves)')
    plt.ylabel('Frequency')
    plt.xticks(range(0, df_excl_n["Length"].max() + 1, 20))
    plt.grid(axis='y', linestyle='--', alpha=0.7)
    plt.tight_layout()
    plt.show()

def plot_avg_turn_length_by_eval(df):
    """Plot average turn length by evaluation combination including rows with Victory = 'N'."""
    grouped = df.groupby(["Attacker Eval", "Defender Eval"])["Length"].mean().reset_index()
    combinations = [(0, 0), (0, 1), (1, 0), (1, 1)]
    avg_lengths = [
        grouped[(grouped["Attacker Eval"] == ae) & (grouped["Defender Eval"] == de)]["Length"].values[0]
        if not grouped[(grouped["Attacker Eval"] == ae) & (grouped["Defender Eval"] == de)].empty else 0
        for ae, de in combinations
    ]
    plt.figure(figsize=(8, 5))
    plt.bar(['(0, 0)', '(0, 1)', '(1, 0)', '(1, 1)'], avg_lengths, color='coral', edgecolor='black')
    plt.title('Average Turn Length by Evaluation Combination (Including Victory "N")')
    plt.xlabel('Evaluation Combination (Attacker Eval, Defender Eval)')
    plt.ylabel('Average Game Length (Number of Moves)')
    plt.grid(axis='y', linestyle='--', alpha=0.7)
    plt.tight_layout()
    plt.show()

def plot_avg_turn_length_by_eval_excl_n(df):
    """Plot average turn length by evaluation combination excluding rows with Victory = 'N'."""
    df_excl_n = df[df["Victory"] != "N"]
    grouped_excl_n = df_excl_n.groupby(["Attacker Eval", "Defender Eval"])["Length"].mean().reset_index()
    combinations = [(0, 0), (0, 1), (1, 0), (1, 1)]
    avg_lengths = [
        grouped_excl_n[(grouped_excl_n["Attacker Eval"] == ae) & (grouped_excl_n["Defender Eval"] == de)]["Length"].values[0]
        if not grouped_excl_n[(grouped_excl_n["Attacker Eval"] == ae) & (grouped_excl_n["Defender Eval"] == de)].empty else 0
        for ae, de in combinations
    ]
    plt.figure(figsize=(8, 5))
    plt.bar(['(0, 0)', '(0, 1)', '(1, 0)', '(1, 1)'], avg_lengths, color='coral', edgecolor='black')
    plt.title('Average Turn Length by Evaluation Combination (Excluding Victory "N")')
    plt.xlabel('Evaluation Combination (Attacker Eval, Defender Eval)')
    plt.ylabel('Average Game Length (Number of Moves)')
    plt.grid(axis='y', linestyle='--', alpha=0.7)
    plt.tight_layout()
    plt.show()

def plot_game_length_by_victory_type(df):
    """Plot game length distribution by victory type, excluding rows with Victory = 'N'."""
    df_filtered = df.iloc[1:][df["Victory"] != "N"]
    victory_grouped = df_filtered.groupby(['Victory', 'Length']).size().reset_index(name='Count')
    plt.figure(figsize=(12, 6))
    # sns.scatterplot(x='Length', y='Count', hue='Victory', data=victory_grouped, marker='o')
    plt.title('Game Length Distribution by Victory Type (Excluding "N")')
    plt.xlabel('Game Length (Number of Moves)')
    plt.ylabel('Frequency')
    plt.xticks(range(0, df_filtered["Length"].max() + 1, 20))
    plt.grid(axis='y', linestyle='--', alpha=0.7)
    plt.tight_layout()
    plt.show()

def plot_time_analysis_by_eval(df):
    """Plot average and slowest time analysis grouped by evaluation metrics."""
    df_filtered = df[(df["Victory"] != "N") & (df["Search Depth"].astype(int) >= 3)]
    for board_type, board_data in df_filtered.groupby("Game File"):
        time_metrics = board_data.groupby(["Attacker Eval", "Defender Eval"])[[
            "Avg Attack Time", "Slowest Attack Time", "Avg Defense Time", "Slowest Defense Time"
        ]].mean().reset_index()
        time_metrics["Eval Pair"] = time_metrics.apply(lambda row: f'({int(row["Attacker Eval"])}, {int(row["Defender Eval"])})', axis=1)
        ax = time_metrics.set_index("Eval Pair").plot(kind='bar', figsize=(12, 6), colormap='viridis', edgecolor='black')
        ax.set_title(f'Average and Slowest Time Analysis by Eval Metrics (Board Type: {board_type})')
        ax.set_xlabel('Evaluation Pair (Attacker Eval, Defender Eval)')
        ax.set_ylabel('Time (Milliseconds)')
        ax.axhline(y=5000, color='red', linewidth=2, linestyle='--', label='5000 ms Threshold')
        ax.grid(axis='y', linestyle='--', alpha=0.7)
        ax.set_xticklabels(ax.get_xticklabels(), rotation=45)
        ax.legend(loc='upper right')
        plt.tight_layout()
        plt.show()

def plot_time_analysis_by_mord(df):
    """Plot average and slowest time analysis grouped by Mord metrics."""
    df_filtered = df[(df["Search Depth"].astype(int) >= 3)] #(df["Victory"] != "N") & to get rid of stall games
    thing = 1
    for board_type, board_data in df_filtered.groupby("Game File"):
        time_metrics = board_data.groupby(["Attacker Mord", "Defender Mord"])[[
            "Avg Attack Time", "Slowest Attack Time", "Avg Defense Time", "Slowest Defense Time"
        ]].mean().reset_index()
        time_metrics["Mord Pair"] = time_metrics.apply(lambda row: f'({int(row["Attacker Mord"])}, {int(row["Defender Mord"])})', axis=1)
        ax = time_metrics.set_index("Mord Pair").plot(kind='bar', figsize=(12, 6), colormap='plasma', edgecolor='black')
        ax.set_title(f'Average and Slowest Time Analysis by Mord Metrics (Board Type: {board_type})')
        ax.set_xlabel('Mord Pair (Attacker Mord, Defender Mord)')
        ax.set_ylabel('Time (Milliseconds)')
        # ax.axhline(y=5000, color='red', linewidth=2, linestyle='--', label='5000 ms Threshold')
        # ax.grid(axis='y', linestyle='--', alpha=0.7)
        ax.set_xticklabels(ax.get_xticklabels(), rotation=45)
        ax.legend(loc='upper right')
        plt.tight_layout()
        file_name = str(thing) + "_mord_info.pdf"
        thing += 1
        plt.savefig(file_name)
        # plt.show()
        plt.close()

def analyze_game_data():
    """Main function to read the data and call the plotting functions."""
    file_path = select_file()
    if not file_path:
        print("No file selected. Exiting.")
        return

    columns = [
        "Game File", "Search Depth", "Victory", "Length",
        "Attacker Eval", "Attacker Mord", "Avg Attack Time", "Slowest Attack Time",
        "Defender Eval", "Defender Mord", "Avg Defense Time", "Slowest Defense Time"
    ]

    # Read the file into a DataFrame and skip extra header rows if present
    df = pd.read_csv(file_path, header=None, names=columns, skiprows=1)

    # Remove leading/trailing whitespace from column names and data
    df.columns = df.columns.str.strip()
    df = df.apply(lambda x: x.str.strip() if x.dtype == "object" else x)

    # Safely convert numeric columns, including Search Depth, to integers or floats
    df["Length"] = pd.to_numeric(df["Length"], errors='coerce').fillna(0).astype(int)
    df["Search Depth"] = pd.to_numeric(df["Search Depth"], errors='coerce').fillna(0).astype(int)
    df["Attacker Eval"] = pd.to_numeric(df["Attacker Eval"], errors='coerce').fillna(0).astype(int)
    df["Defender Eval"] = pd.to_numeric(df["Defender Eval"], errors='coerce').fillna(0).astype(int)
    df["Attacker Mord"] = pd.to_numeric(df["Attacker Mord"], errors = 'coerce').fillna(0).astype(int)
    df["Defender Mord"] = pd.to_numeric(df["Defender Mord"], errors = 'coerce').fillna(0).astype(int)
    df["Avg Attack Time"] = pd.to_numeric(df["Avg Attack Time"], errors = 'coerce').fillna(0).astype(int)
    df["Slowest Attack Time"] = pd.to_numeric(df["Slowest Attack Time"], errors = 'coerce').fillna(0).astype(int)
    df["Avg Defense Time"] = pd.to_numeric(df["Avg Defense Time"], errors = 'coerce').fillna(0).astype(int)
    df["Slowest Defense Time"] = pd.to_numeric(df["Slowest Defense Time"], errors = 'coerce').fillna(0).astype(int)
    
    
    

    # Call each plotting function
    # scatter_time_by_mord(df)
    #plot_game_length_distribution_including_n(df)
    #plot_game_length_distribution_excluding_n(df)
    #plot_avg_turn_length_by_eval_excl_n(df)
    # plot_game_length_by_victory_type(df)
    #plot_time_analysis_by_eval(df)
    plot_time_analysis_by_mord(df)


# Run the analysis
analyze_game_data()