#!/usr/bin/env python3
import numpy as np
import matplotlib.pyplot as plt
import json
import re
import glob
from matplotlib.colors import hsv_to_rgb

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
    parse_filename(f) : json.load(open(f))
        for f in files
}

string_lens, string_counts, sep_lens = set(), set(), set()

for key in data.keys():
    string_len, string_count, sep_len, _ = key
    string_lens.add(int(string_len))
    string_counts.add(int(string_count))
    sep_lens.add(int(sep_len))

def sorted_list(coll):
    l = list(coll)
    l.sort()
    return np.array(l)

string_lens = sorted_list(string_lens)
string_counts = sorted_list(string_counts)
sep_lens = sorted_list(sep_lens)

# -> (old, new)
def parameters_to_filenames(string_len, string_count, sep_len) -> (str, str):
    s = lambda bench_name: f'target/criterion/len:{string_len}_n:{string_count}_sep_len:{sep_len}/{bench_name}/new/estimates.json'
    return (s('old'), s('new'))

#fig = plt.figure()

#plt.plot(string_counts, [data[10, n_str, 4]['Mean']['point_estimate'] for n_str in string_counts])

plot = True

# vary colors by brightness (value) and hue
# hue for string len
# value for sep_len
# equidistant

value_step = 1 / len(sep_lens)
hue_step = 1 / len(string_lens)

fig0: plt.Figure = plt.figure(0)
for line_style, (steps_val, sep_len) in zip(['-', '--', ':'], enumerate(sep_lens)):
    # brightness in hsv
    #v = 1 - (steps_val * value_step)/2
    for steps_hue, string_len in enumerate(string_lens):
        h = steps_hue * hue_step
        # Map[(string_len, n_strings, sep_len, tag), json]
        get_data_ = lambda n_str, tag: data[string_len, n_str, sep_len, tag]['Mean']['point_estimate']
        speedup = lambda n_str: get_data_(n_str, 'old') / get_data_(n_str, 'new')

        if not plot:
            print(f'\nstr_len: {string_len}, sep_len: {sep_len}')
            for n_str in string_counts:
                print(f'\t{n_str:5} {speedup(n_str)}')
        else:
            line = plt.plot(
                string_counts,
                [speedup(n_str) for n_str in string_counts],
                '.',
                color = hsv_to_rgb([h, 1, 1]),
                linestyle = line_style,
                label = f'{sep_len:4}, {string_len}'
            )

#plt.plot(string_counts[[0, -1]] + [-50, 100000], [1, 1], color='black', linewidth=2)
ax: plt.Axes = plt.axes()
ax.set_xlabel('N_strings')
ax.set_adjustable('box-forced')
ax.set_xscale('log')
ax.grid('on')
ax.set_ylabel('Speedup')
legend0 = ax.legend(loc='center left', bbox_to_anchor=(1.05, 0.5),
          fancybox=True, shadow=True)
#fig0.bbox_inches = 'tight'

plt.figure(1)
for line_style, (steps_val, sep_len) in zip(['-', '--', ':'], enumerate(sep_lens)):
    # brightness in hsv
    #v = 1 - (steps_val * value_step)/2
    for steps_hue, n_str in enumerate(string_counts):
        h = steps_hue * hue_step
        # Map[(string_len, n_strings, sep_len, tag), json]
        get_data_ = lambda len_string, tag: data[len_string, n_str, sep_len, tag]['Mean']['point_estimate']
        speedup = lambda len_string: get_data_(len_string, 'old') / get_data_(len_string, 'new')

        if not plot:
            pass
            # Not implemented yet
            #assert False
            #print(f'\nstr_len: {string_len}, sep_len: {sep_len}')
            #for n_str in string_counts:
            #    print(f'\t{n_str:5} {speedup(n_str)}')
        else:
            line = plt.plot(
                string_lens,
                [speedup(len_string) for len_string in string_lens],
                '.',
                color = hsv_to_rgb([h, 1, 1]),
                linestyle = line_style,
                label = f'len_sep: {sep_len:4} n_strings: {n_str}'
            )
plt.figure(1, bbox_inches = 'tight')

if plot:
    # figure(1) specific
    ax = plt.axes()
    ax.set_xscale('log')
    ax.set_xlabel('Length_string')
    ax.set_ylabel('Speedup')
    ax.grid('on')

    ax.legend(loc='center left', bbox_to_anchor=(1.05, 0.5),
            fancybox=True, shadow=True)

if plot:
    plt.show()

def get_data(str_len, n_str, sep_len, tag) -> float:
    return data[string_len, n_str, sep_len, tag]['Mean']['point_estimate']
