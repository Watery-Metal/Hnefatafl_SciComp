import pandas as pd
import matplotlib.pyplot as plt
from tkinter import Tk, filedialog
import seaborn as sns
import os

def select_file():
    """Open a file dialog to select a file."""
    root = Tk()
    root.withdraw()  # Hide the root window
    file_path = filedialog.askopenfilename(
        title="Select a Text File",
        filetypes=[("Text Files", "*.txt"), ("CSV Files", "*.csv"), ("All Files", "*.*")]
    )
    return file_path

def game_length_distribution(df, file_name):
    #Plot the length of games with respect to search depth
    depth_groups = df.groupby(['Search Depth', 'Length']).size().reset_index(name='Count')
    plt.figure()
    plt.title("Game lengths with respect to Search Depth")
    sns.scatterplot(x='Search Depth', y='Count', hue='Search Depth', data=depth_groups, marker='o')
    plt.xlabel("Search Depth")
    plt.ylabel("Turns Taken")
    plt.xticks([1,2,3])
    game_length_filename = file_name + "_game_length_visualization.pdf"
    # plt.savefig(game_length_filename)
    plt.show()
    plt.close()

    #Alternatively, an overlapping bar graph, showing the skew
    grouped_counts = df.groupby(["Search Depth", "Length"]).size().reset_index(name="Count")
    pivot_data = grouped_counts.pivot(index="Length", columns="Search Depth", values="Count").fillna(0)
    ax = pivot_data.plot(kind="bar", stacked=True, colormap='viridis', edgecolor="black")
    plt.title("Game Lengths By Search Depths")
    plt.xlabel("In-Game Turns")
    plt.ylabel("Occurences in Test")
    plt.xticks()
    plt.show()
    game_length_filename2 = file_name + "_game_length_barchart.pdf"
    # plt.savefig(game_length_filename2)
    plt.close()

def win_totals(df, file_name):
    # Visualize the frequency of win conditions with respect to various parameters

    # Temporary change to include only algorithm zero
    # df = df[df["Attacker Eval"] != 1]
    # df = df[df["Defender Eval"] != 1]

    #Temporary change to include only depth four search
    # df = df[df["Search Depth"] == 4]

    # Search Depth Distribution
    sd_victory = df.groupby(["Victory", "Search Depth"]).size().reset_index(name="Count")
    pivot_data = sd_victory.pivot(index="Search Depth", columns="Victory", values="Count").fillna(0)
    pivot_data.plot(kind="bar", stacked=True, edgecolor="black")
    plt.title("Win Conditions with Respect to Search Depth (Standard Boards)")
    plt.ylabel("Occurences")
    # plt.show()
    sdwintotal = file_name + "_win_totals_sd.png"
    plt.savefig(sdwintotal)
    plt.close()

    # Algorithm Choice analysis
    df_excl_n = df[df["Victory"] != "N"] #This ignores all stall games

    attacker_nostall = df_excl_n.groupby(["Victory", "Attacker Eval"]).size().reset_index(name="Count")
    attack_pivot = attacker_nostall.pivot(index="Attacker Eval", columns="Victory", values="Count").fillna(0)
    attack_pivot.plot(kind="bar", stacked=True, edgecolor="black")
    plt.ylabel("Instances of Victory")
    plt.title("Victory Conditions by Attacker Evaluation")
    plt.show()
    attacker_wins = file_name + "_attack_evals.png"
    # plt.savefig(attacker_wins)
    plt.close()

    defender_nostall = df_excl_n.groupby(["Victory", "Defender Eval"]).size().reset_index(name="Count")
    defender_pivot = defender_nostall.pivot(index="Defender Eval", columns="Victory", values="Count").fillna(0)
    defender_pivot.plot(kind="bar", stacked=True, edgecolor="black")
    plt.ylabel("Instances of Victory")
    plt.title("Victory Conditions by Defender Evaluation")
    plt.show()
    defender_wins = file_name + "_defend_evals.png"
    # plt.savefig(defender_wins)
    plt.close()


def mord_analysis(df):
    # Potential Differences in Execution Time According to Mord

    # Filter out eval one
    df = df[(df["Attacker Eval"] != 1)]
    df = df[(df["Defender Eval"] != 1)]
    # For now, we only consider Search Depth Three or less (We want largely deterministic results only)
    df = df[(df["Search Depth"].astype(int) <= 3)]

    for board_type, board_data in df.groupby("Game File"):
        board_data = board_data[board_data["Game File"] == board_type]
        deeper_mord_analysis(board_data, board_type)



def deeper_mord_analysis(df, file_name):
    grouped_amord = df.groupby(["Attacker Mord", "Avg Attack Time", "Search Depth"]).size().reset_index(name='Count')
    sns.scatterplot(data=grouped_amord, x="Attacker Mord", y="Avg Attack Time", hue="Search Depth", palette="deep")
    plt.title(f"Attacker Response by Movement-Ordering ({file_name})")
    mordname1 = file_name + "_attacker_filtered_final_mord_information.png"
    plt.ylabel("Execution (ms)")
    plt.xlabel("Movement Ordering")
    plt.savefig(mordname1)
    # plt.show()
    plt.close()

    grouped_dmord = df.groupby(["Defender Mord", "Avg Defense Time", "Search Depth"]).size().reset_index(name='Count')
    sns.scatterplot(data=grouped_dmord, x="Defender Mord", y="Avg Defense Time", hue="Search Depth", markers='h', palette="deep")
    plt.title(f"Defender Response by Movement-Ordering ({file_name})")
    plt.ylabel("Execution (ms)")
    plt.xlabel("Movement Ordering")
    mordname2 = file_name + "_defender_filtered_final_mord_information.png"
    plt.savefig(mordname2)
    # plt.show()
    plt.close()

def weighted_mord_average(df):
    # Prints the average response in ms by mord, weighted by turns
    
    # Filter data
    # df = df[(df["Attacker Eval"] == 2)]
    df = df[(df["Defender Eval"] == 2)]
    # For now, we only consider Search Depth Three or less (We want largely deterministic results only)
    df = df[(df["Search Depth"].astype(int) == 3)]

    averages_att = df.groupby("Attacker Mord").apply(weighted_average, 'Avg Attack Time', 'Length')
    averages_def = df.groupby("Defender Mord").apply(weighted_average, 'Avg Defense Time', 'Length')

    print(averages_att, averages_def)


def weighted_average(dataframe, value, weight):
    val = dataframe[value]
    wt = dataframe[weight]
    return (val * wt).sum() / wt.sum()


def defender_win_percentage(df):
    # Computes for a given dataset how often 'K' or 'A' are the Victory Conditions
    
    # Filter Data if desired
    df = df[df["Attacker Eval"].astype(int) == 2]
    # df = df[df["Defender Eval"].astype(int) == 1]
    df = df[df["Search Depth"].astype(int) == 4]

    # We could Use more clever conditional filtering, but this is a single purpose script
    non_stall_victories = df[df["Victory"] != "N"].count()["Victory"]
    attacker_extinction_count = df[df["Victory"] == "A"].count()["Victory"]
    king_in_corner_count = df[df["Victory"] == "K"].count()["Victory"]

    defense_success_percentage = 100 * ((attacker_extinction_count + king_in_corner_count) / non_stall_victories)
    print(f"Defense Win Percentage of Supplied Dataset: {defense_success_percentage}")
    # print(f"Common sense check. I have {non_stall_victories} non-stalled games, and {attacker_extinction_count} attacker extinctions")

def analyze_game_data():
    """Main function to read the data and call the plotting functions."""
    file_path = select_file()
    if not file_path:
        print("No file selected. Exiting.")
        return
    
    file_name, file_extension = os.path.splitext(file_path)

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
    
    # game_length_distribution(df, file_name)
    # mord_analysis(df)
    # weighted_mord_average(df)
    # win_totals(df, file_name)
    # defender_win_percentage(df)


# Run the analysis
analyze_game_data()

