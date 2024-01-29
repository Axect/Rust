import matplotlib.pyplot as plt
from matplotlib import ticker
import scienceplots
import numpy as np

# Prepare Data to Plot
dummy = np.array([0, 1, 2])
dbs = ["JammDB", "NativeDB", "Files+netCDF"]
data = {
    'Read': [2.4, 9.4, 98.4],
    'Update': [8.9, 11.6, 99.0],
    'Write': [371.3, 882.0, 17.2 * 1000],
}
size = [200.1, 137.5, 1.5 * 1000]

width = 0.25
multiplier = 0

# Plot params
pparam = dict(
    ylabel = r'Time (ms)',
    title = r"Benchmark for Database",
    xscale = 'linear',
    yscale = 'log',
    ylim = (1, 40000)
)

# Plot
with plt.style.context(["science", "nature"]):
    fig, ax = plt.subplots()
    ax.autoscale(tight=True)
    ax.set(**pparam)
    for name, values in data.items():
        offset = width * multiplier
        rects = ax.bar(dummy + offset, values, width, label=name)
        ax.bar_label(rects, padding=3)
        multiplier += 1
    ax.legend()
    ax.set_xticks(dummy + width, dbs)
    # Disable minor ticks
    ax.get_xaxis().set_minor_locator(ticker.NullLocator())
    fig.savefig('time_plot.png', dpi=600, bbox_inches='tight')
