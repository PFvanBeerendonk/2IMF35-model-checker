import os
import re
import matplotlib.pyplot as plt
import numpy as np

def parse_file(file_path):
    with open(file_path, 'r', encoding='utf-8') as file:
        lines = file.readlines()

    time_pattern = re.compile(r'Time:\s+([\d.]+)([μµ]?s|ms|s)')
    lifts_pattern = re.compile(r'Nr of Lifts:\s+(\d+)')
    success_lifts_pattern = re.compile(r'Success lifts:\s+(\d+)')

    time_value = None
    lifts = None
    success_lifts = None

    for line in lines:
        # Normalize the micro symbol to 'µ'
        line = line.replace('Âµ', 'µ')
        
        time_match = time_pattern.match(line)
        lifts_match = lifts_pattern.match(line)
        success_lifts_match = success_lifts_pattern.match(line)

        if time_match:
            time_value = float(time_match.group(1))
            unit = time_match.group(2)

            # Convert time to seconds if it's in microseconds or milliseconds
            if unit == 'μs' or unit == 'µs':
                time_value /= 1e6
            elif unit == 'ms':
                time_value /= 1e3

        if lifts_match:
            lifts = int(lifts_match.group(1))

        if success_lifts_match:
            success_lifts = int(success_lifts_match.group(1))

    return time_value, lifts, success_lifts

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

def create_time_bar_chart(folder_path, file_name):
    data = {}

    for file in os.listdir(folder_path):
        if file.endswith(".txt") and "lifting" in file and file_name in file:
            k = int(re.search(r'lifting-(\d)', file).group(1))
            file_path = os.path.join(folder_path, file)
            time, _, _ = parse_file(file_path)

            if time is not None:
                data.setdefault(k, []).append(time)

    k_values = list(range(6))  # Values of k range from 0 to 5
    k_values.sort()

    bar_width = 0.2
    opacity = 0.8
    color_map = plt.get_cmap('tab10', len(k_values))  # Use 'tab10' colormap for distinct colors

    for j, k_value in enumerate(k_values):
        if k_value in data:
            plt.bar(k_value, np.mean(data[k_value]), bar_width, alpha=opacity, label=f'k={k_value} ({k_description(k_value)})', color=color_map(j))

    plt.xlabel('Lifting Strategies')
    plt.ylabel('Average Time (s)')
    plt.title(f'Bar Chart for Time - {file_name}')
    plt.legend(title='Lifting Strategies', loc='upper right', bbox_to_anchor=(1.05, 1))  # Adjust the position of the legend

    # Save the plot to the "graphs" folder
    save_path = os.path.join(os.getcwd(), 'graphs', f'time_bar_chart_{file_name}.png')
    os.makedirs(os.path.dirname(save_path), exist_ok=True)
    plt.savefig(save_path, bbox_inches='tight')
    plt.close()

def create_lifts_bar_chart(folder_path, file_name):
    data = {}

    for file in os.listdir(folder_path):
        if file.endswith(".txt") and "lifting" in file and file_name in file:
            k = int(re.search(r'lifting-(\d)', file).group(1))
            file_path = os.path.join(folder_path, file)
            _, lifts, success_lifts = parse_file(file_path)

            if lifts is not None and success_lifts is not None:
                data.setdefault(k, []).append((lifts, success_lifts))

    k_values = list(range(6))  # Values of k range from 0 to 5
    k_values.sort()

    bar_width = 0.2
    opacity = 0.8
    color_map = plt.get_cmap('tab10', len(k_values))  # Use 'tab10' colormap for distinct colors

    for j, k_value in enumerate(k_values):
        if k_value in data:
            values = np.array(data[k_value])
            avg_lifts = np.mean(values[:, 0])
            avg_success_lifts = np.mean(values[:, 1])

            position = j * bar_width

            plt.bar(position - bar_width / 2, avg_lifts, bar_width, alpha=opacity/2, color='grey', label='Avg Nr of Lifts' if j == 0 else '')
            plt.bar(position + bar_width / 2, avg_success_lifts, bar_width, alpha=opacity/2, color='green', label='Avg Success Lifts' if j == 0 else '')

    plt.xlabel('Lifting Strategies')
    plt.ylabel('Average Number of Lifts')
    plt.title(f'Bar Chart for Number of Lifts - {file_name}')
    plt.legend(title='Lift Statistics', loc='upper right', bbox_to_anchor=(1.05, 1))  # Adjust the position of the legend

    # Save the plot to the "graphs" folder
    save_path = os.path.join(os.getcwd(), 'graphs', f'lifts_bar_chart_{file_name}.png')
    os.makedirs(os.path.dirname(save_path), exist_ok=True)
    plt.savefig(save_path, bbox_inches='tight')
    plt.close()

if __name__ == "__main__":
    folder_path = "./../input/testcases/results/"  # Replace with the actual folder path

    for file_name in set(file.split("-lifting")[0] for file in os.listdir(folder_path) if file.endswith(".txt") and "lifting" in file):
        create_time_bar_chart(folder_path, file_name)
