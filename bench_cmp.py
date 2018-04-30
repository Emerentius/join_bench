#!/usr/bin/env python3
import numpy as np
import matplotlib.pyplot as plt
import json
import re
import glob
from matplotlib.colors import hsv_to_rgb
import pandas as pd
from typing import Tuple
import sys

files = glob.glob('target/criterion/len:*_n:*_sep_len:*/*_join/new/estimates.json')
pattern = re.compile(r'target/criterion/len:(\d+)_n:(\d+)_sep_len:(\d+)/([a-zA-Z]+)_join/new/estimates.json')
match = pattern.match('')
#match.
# Map[(string_len, n_strings, sep_len, tag), json]
def parse_filename(name) -> (int, int, int, str):
    matches = pattern.match(name).groups()
    ints = tuple(int(n) for n in matches[:3])
    tag = matches[3]
    return ints + (tag,)

data = {
    parse_filename(f) : json.load(open(f))['Mean']['point_estimate']
        for f in files
}
df = pd.DataFrame([*k, v ] for k, v in data.items())
df.columns = ['len_string', 'n_strings', 'len_sep', 'tag', 'time']
df.reindex(columns = ['len_sep', 'len_string', 'n_strings', 'tag', 'time'])
df.sort_values(list(df.columns))

sorted_deduplicated = lambda coll: np.array(sorted(set(coll)))

string_lens = sorted_deduplicated(df['len_string'])
string_counts = sorted_deduplicated(df['n_strings'])
sep_lens = sorted_deduplicated(df['len_sep'])

###########
#should_plot = True
###########

# speedup_fn = speedup(dim_ls, dim_hue, dim_x) -> float
# dim_x = dimension x-axis
# label_fn = label(dim_ls, dim_hue)
# NOTE: made outer loop the color loop, inner one the style loop
#       also explicitly chose colors rather than using numerical hues
#       didn't change names or documentation
def plot(dim_linestyle, dim_hue, dim_x, speedup_fn, label_fn) -> Tuple[plt.Axes, plt.legend]:
    for color, dim_ls in zip(['red', 'blue', 'black', 'green'], dim_linestyle):
        for line_style, dim_h in zip(['-', '--', ':', '-.', ' '], dim_hue):
            #h = steps_hue * hue_step
            #get_data_ = lambda len_string, tag: data[len_string, n_str, sep_len, tag]
            #speedup = lambda len_string: get_data_(len_string, 'old') / get_data_(len_string, 'new')

            plt.plot(
                dim_x,
                [speedup(dim_ls, dim_h, dim) for dim in dim_x],
                '.',
                color = color,
                linestyle = line_style,
                label = label_fn(dim_ls, dim_h)
            )

    ax = plt.axes()
    ax.set_xscale('log')
    ax.set_ylabel('Speedup')
    ax.grid('on')

    lg = ax.legend(loc='center left', bbox_to_anchor=(1.01, 0.5), fancybox=True, shadow=True)
    return ax, lg

def print_speedups():
    print(f'{"len_separator":>12}{"len_string":>12}{"n_strings":>12}{"speedup":>12}')
    for sep_len in sep_lens:
        for string_len in string_lens:
            # Map[(string_len, n_strings, sep_len, tag), json]
            get_data_ = lambda n_str, tag: data[string_len, n_str, sep_len, tag]
            speedup = lambda n_str: get_data_(n_str, 'old') / get_data_(n_str, 'new')

            for n_str in string_counts:
                print(f'{sep_len:12}{string_len:12}{n_str:12}{speedup(n_str):12.3}')


# vary colors by hue
# equidistant

###############################################################################################
# copy-paste code ftw
###############################################################################################
fig0: plt.Figure = plt.figure(0)

get_data_ = lambda len_sep, len_string, n_str, tag: data[len_string, n_str, len_sep, tag]
speedup = lambda *args: get_data_(*args, 'old') / get_data_(*args, 'new')
label_maker = lambda len_sep, len_string: f'{len_sep:>4}, {len_string:>5}'

ax0, lg0 = plot(sep_lens, string_lens, string_counts, speedup, label_maker)
ax0.set_xlabel('Number of Joined Strings')

lg0.set_title('separator len, string len')


###############################################################################################
fig1 = plt.figure(1)
get_data_ = lambda len_sep, n_str, len_string, tag: data[len_string, n_str, len_sep, tag]
speedup = lambda *args: get_data_(*args, 'old') / get_data_(*args, 'new')
label_maker = lambda len_sep, n_str: f'{len_sep:>4}, {n_str:>5}'

ax1, lg1 = plot(sep_lens, string_counts, string_lens, speedup, label_maker)
ax1.set_xlabel('Length of Strings that are joined')

lg1.set_title('separator len, num of strings')


###############################################################################################
fig2 = plt.figure(2)
get_data_ = lambda len_string, n_str, len_sep, tag: data[len_string, n_str, len_sep, tag]
speedup = lambda *args: get_data_(*args, 'old') / get_data_(*args, 'new')
label_maker = lambda len_str, n_str: f'{len_str:>4}, {n_str: >5}'

ax2, lg2 = plot(string_lens, string_counts, sep_lens, speedup, label_maker)
ax2.set_xlabel('Separator Length')

lg2.set_title('string len, num of strings')

try:
    assert len(sys.argv) == 2
    arg = sys.argv[1]
    if arg == 'show':
        plt.show()
    elif arg == 'save':
        fig0.savefig('speedup_n_strings.png', bbox_inches='tight')
        fig1.savefig('speedup_len_string.png', bbox_inches='tight')
        fig2.savefig('speedup_len_separator.png', bbox_inches='tight')
    elif arg == 'print':
        print_speedups()
except:
    print(f'usage: {sys.argv[0]} (show | save | print)')

#plt.show()
