import os
import re
import matplotlib.pyplot as plt
import numpy as np

def parse_file(file_path):
    with open(file_path, 'r') as file:
        lines = file.readlines()

    size_pattern = re.compile(r'Size:\s+(\d+)')
    non_t_pattern = re.compile(r'Nr of non-T:\s+(\d+)')
    t_pattern = re.compile(r'Nr of T:\s+(\d+)')
    time_pattern = re.compile(r'Time:\s+([\d.]+)(ms|s)')
    lifts_pattern = re.compile(r'Nr of Lifts:\s+(\d+)')
    success_lifts_pattern = re.compile(r'Success lifts:\s+(\d+)')

    size = None
    non_t = None
    t = None
    time_value = None
    lifts = None
    success_lifts = None

    for line in lines:
        size_match = size_pattern.match(line)
        non_t_match = non_t_pattern.match(line)
        t_match = t_pattern.match(line)
        time_match = time_pattern.match(line)
        lifts_match = lifts_pattern.match(line)
        success_lifts_match = success_lifts_pattern.match(line)

        if size_match:
            size = int(size_match.group(1))

        if non_t_match:
            non_t = int(non_t_match.group(1))

        if t_match:
            t = int(t_match.group(1))

        if time_match:
            time_value = float(time_match.group(1))
            unit = time_match.group(2)

            # Convert time to seconds if it's in milliseconds
            if unit == 'ms':
                time_value /= 1000.0

        if lifts_match:
            lifts = int(lifts_match.group(1))

        if success_lifts_match:
            success_lifts = int(success_lifts_match.group(1))

    return size, non_t, t, time_value, lifts, success_lifts

def extract_file_name(file_name):
    # Remove spaces and escape underscores
    file_name = file_name.replace(" ", "").replace("_", r"\_")
    match = re.search(r'^(.*?)(\d+)(.*)-lifting-\d', file_name)
    if match:
        return f"{match.group(2)}-{match.group(3)}"
    return None

def k_description(k_value):
    descriptions = [
        "Input Order",
        "Random Order",
        "Least Successors",
        "Most Successors",
        "Predecessors",
        "Focus List"
    ]
    return descriptions[k_value] if 0 <= k_value < len(descriptions) else f"Unknown ({k_value})"

def create_time_bar_chart(folder_path):
    data = {}
    t_values_arr = []
    non_t_values_arr = []

    for file_name in os.listdir(folder_path):
        if file_name.endswith(".txt") and "lifting" in file_name:
            k = int(re.search(r'lifting-(\d)', file_name).group(1))
            file_path = os.path.join(folder_path, file_name)
            size, non_t, t, time, _, _ = parse_file(file_path)

            if size is not None and non_t is not None and t is not None and time is not None:
                file_name_to_show = extract_file_name(file_name)
                if file_name_to_show is not None:
                    data.setdefault(file_name_to_show, []).append((k, size, non_t, t, time))

    file_names = list(data.keys())
    k_values = list(range(6))  # Values of k range from 0 to 5
    k_values.sort()

    bar_width = 0.2
    group_gap = 0.4
    opacity = 0.8
    color_map = plt.get_cmap('tab10', len(k_values))  # Use 'tab10' colormap for distinct colors

    for j, k_value in enumerate(k_values):
        legend_label_added = False

        for i, file_name_to_show in enumerate(file_names):
            values = np.array(data[file_name_to_show])
            times = values[:, 4].astype(float)  # Ensure times is a NumPy array of floats

            position = i * (len(k_values) * bar_width + group_gap) + j * bar_width

            if not legend_label_added and j < len(times) and not np.isnan(times[j]):
                plt.bar(position, times[j], bar_width, alpha=opacity, label=f'k={k_value} ({k_description(k_value)})', color=color_map(j))
                legend_label_added = True
            elif j < len(times):
                plt.bar(position, times[j], bar_width, alpha=opacity, color=color_map(j))

    plt.xlabel('File Names')
    plt.ylabel('Time (s)')
    plt.title('Grouped Bar Chart for Time based on File Names and Lifting Strategies')
    x_positions = [i * (len(k_values) * bar_width + group_gap) + ((len(k_values) * bar_width) / 2) for i in range(len(file_names))]
    plt.xticks(x_positions, file_names, rotation=45, ha='right')  # Rotate labels for better readability
    plt.legend(title='Lifting Strategies', loc='upper right', bbox_to_anchor=(1.3, 1))
    plt.tight_layout()
    plt.show()

    # Check and print the number of T and non-T for each file
    for file_name_to_show in file_names:
        values = np.array(data[file_name_to_show])
        non_t_values = values[:, 2]
        t_values = values[:, 3]

        if not np.all(non_t_values == non_t_values[0]) or not np.all(t_values == t_values[0]):
            print(f"ERROR: Number of T and non-T are not the same for all lifting strategies in file: {file_name_to_show}")
        else:
            print(f"Number of T for {file_name_to_show}: {t_values[0]}")
            print(f"Number of non-T for {file_name_to_show}: {non_t_values[0]}")
            t_values_arr.append(t_values[0])
            non_t_values_arr.append(non_t_values[0])

    generate_latex_table(file_names, t_values_arr, non_t_values_arr)

def create_lifts_bar_chart(folder_path):
    data = {}

    for file_name in os.listdir(folder_path):
        if file_name.endswith(".txt") and "lifting" in file_name:
            k = int(re.search(r'lifting-(\d)', file_name).group(1))
            file_path = os.path.join(folder_path, file_name)
            _, _, _, _, lifts, success_lifts = parse_file(file_path)

            if lifts is not None and success_lifts is not None:
                file_name_to_show = extract_file_name(file_name)
                if file_name_to_show is not None:
                    data.setdefault(file_name_to_show, []).append((k, lifts, success_lifts))

    file_names = list(data.keys())
    k_values = list(range(6))  # Values of k range from 0 to 5
    k_values.sort()

    bar_width = 0.2
    group_gap = 0.4
    opacity = 0.8
    color_map = plt.get_cmap('tab10', len(k_values))  # Use 'tab10' colormap for distinct colors

    for j, k_value in enumerate(k_values):
        for i, file_name_to_show in enumerate(file_names):
            values = np.array(data[file_name_to_show])
            lifts = values[:, 1]
            success_lifts = values[:, 2]

            position = i * (len(k_values) * bar_width + group_gap) + j * bar_width

            plt.bar(position - bar_width / 2, lifts[j], bar_width, alpha=opacity/2, color='grey', label='Nr of Lifts' if j == 0 else '')
            plt.bar(position + bar_width / 2, success_lifts[j], bar_width, alpha=opacity/2, color='green', label='Success Lifts' if j == 0 else '')

    plt.xlabel('File Names')
    plt.ylabel('Number of Lifts')
    plt.title('Grouped Bar Chart for Number of Lifts and Successful Lifts based on File Names and Lifting Strategies')
    x_positions = [i * (len(k_values) * bar_width + group_gap) + ((len(k_values) * bar_width) / 2) for i in range(len(file_names))]
    plt.xticks(x_positions, file_names, rotation=45, ha='right')  # Rotate labels for better readability
    plt.legend(title='Lift Statistics', loc='upper right', bbox_to_anchor=(1.3, 1))
    plt.tight_layout()
    plt.show()

def generate_latex_table(file_names, t_values, non_t_values):
    print(r"\begin{table}[]")
    print(r"\centering")
    print(r"\begin{tabular}{l|ll}")
    print(r"Elevator games & Number of T & Number of non-T \\ \hline")

    for i in range(len(file_names)):
        print(f"{file_names[i]} & {t_values[i]} & {non_t_values[i]} \\\\")

    print(r"\end{tabular}")
    print(r"\end{table}")

if __name__ == "__main__":
    folder_path = "./../output4/dining_games/"  # Replace with the actual folder path
    create_time_bar_chart(folder_path)
