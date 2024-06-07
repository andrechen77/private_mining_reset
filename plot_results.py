import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D
import numpy as np
import math

# read and parse the file into `data`
file_path = "result.txt"
data = {} # data[conf_depth][(alpha, rdp)] = [(policy, expected_steps)]
with open(file_path, 'r') as file:
    for line in file:
        # Split the line into parts
        alpha, rate_delay_product, reset_policy, expected_steps = eval(line)
        conf_depth = len(reset_policy) - 1

        data.setdefault(conf_depth, {}).setdefault((alpha, rate_delay_product), []).append((reset_policy, expected_steps))

# find the best policy and its associated expected number of steps for every
# configuration. rdp means rate-delay product
best_reset_policies_dict = {} # best_reset_policies[conf_depth][(alpha, rdp)] = (best_policy, best_expected_steps)
for conf_depth, conf_depth_data in data.items():
    for alpha_rdp, trials in conf_depth_data.items():
        best = min(trials, key=lambda item: item[1])
        best_reset_policies_dict.setdefault(conf_depth, {})[alpha_rdp] = best

# plot all the best reset policies
num_plots = len(best_reset_policies_dict)
plots_per_row = 3
num_rows = math.ceil(num_plots / plots_per_row)
fig, axes = plt.subplots(num_rows, plots_per_row, figsize=(plots_per_row * 3, num_rows * 3))
for (conf_depth, conf_depth_data), ax in zip(sorted(best_reset_policies_dict.items(), key=lambda x: x[0]), axes.flat):
    for (alpha, rate_delay_product), (policy, best_expected_steps) in conf_depth_data.items():
        if rate_delay_product != 0.0:
            continue
        ax.plot(range(len(policy)), policy, marker='o', label=f'{alpha} {conf_depth}', c=plt.cm.rainbow(alpha))
    ax.set_xlabel('h_A')
    ax.set_ylabel('reset threshold (h\')')
    ax.set_title(f'k = {conf_depth}')
    # ax.legend()
    ax.grid(True)
    ax.set_xticks(range(conf_depth + 1))
    ax.set_yticks(range(6))
fig.subplots_adjust(wspace=0.5, hspace=0.5)

# plot the best reset policy for selected k, alpha, and rdp
best_policy, best_expected_steps = best_reset_policies_dict[5][(0.3, 0.0)]
plt.plot(range(len(best_policy)), best_policy, marker='o', label=f'{alpha} {conf_depth}', c=plt.cm.rainbow(alpha))
plt.xlabel('h_A')
plt.ylabel('reset threshold (h\')')
plt.title(f'Best policy for k = 5 and alpha = 0.3')
plt.grid(True)
plt.xticks(range(6))
plt.yticks(range(6))
print(f"{best_expected_steps=}")

# plot the expected number of steps over alpha and rdp, grouped by k
num_plots = len(best_reset_policies_dict)
plots_per_row = 3
num_rows = math.ceil(num_plots / plots_per_row)
fig = plt.figure(figsize=(plots_per_row * 5, num_rows * 5))
for i, (conf_depth, conf_depth_data) in enumerate(sorted(best_reset_policies_dict.items(), key=lambda x: x[0])):
    ax = fig.add_subplot(num_rows, plots_per_row, i + 1, projection='3d')

    # prepare the points
    points = sorted([(alpha, rdp, exp_time) for (alpha, rdp), (_, exp_time) in conf_depth_data.items()])
    alphas = np.array([x for x, _, _ in points])
    num_alphas = len(np.unique(alphas))
    rdps = np.array([y for _, y, _ in points])
    num_rdps = len(np.unique(rdps))
    grid_alphas = alphas.reshape((num_alphas, num_rdps))
    grid_rdps = rdps.reshape((num_alphas, num_rdps))
    grid_exp_times = np.array([z for _, _, z in points]).reshape((num_alphas, num_rdps))

    # plot the points
    ax.plot_surface(grid_alphas, grid_rdps, grid_exp_times, linewidth=0)
    ax.set_xlabel('alpha')
    ax.set_ylabel('rate-delay product')
    ax.set_zlabel('best expected time')
    ax.set_title(f'k = {conf_depth}')

# plot the expected number of steps over rdp for selected k and alpha
points = []
for (alpha, rdp), (best_policy, best_expected_steps) in best_reset_policies_dict[5].items():
    if alpha == 0.3:
        points.append((rdp, best_expected_steps))
points = sorted(points)
plt.plot([rdp for rdp, _ in points], [time for _, time in points], marker='o')
plt.xlabel('rate-delay product')
plt.ylabel('expected steps until violation')
plt.title(f'Expected steps for k = 5 and alpha = 0.3')
