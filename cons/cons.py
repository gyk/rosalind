from sys import stdin

import numpy as np

nuc_to_id = {
    'A': 0,
    'C': 1,
    'G': 2,
    'T': 3,
}

id_to_nuc = { v:k for (k, v) in nuc_to_id.iteritems() }.values()

def solve(arr):
    profile_matrix_t = np.array([np.bincount(col, minlength=4)
        for col in arr.transpose()])
    consensus_string_t = np.array([id_to_nuc[np.argmax(col)]
        for col in profile_matrix_t])

    return (consensus_string_t.transpose(), profile_matrix_t.transpose())

def print_consensus_string(s):
    print "".join(s)

def print_profile_matrix(m):
    for (nuc, row) in zip(id_to_nuc, m):
        print "{}: {}".format(nuc, " ".join(map(str, row)))

if __name__ == '__main__':
    dna_list = []

    def read_dna_text():
        dna = []
        for line in stdin:
            if line.startswith('>'):
                if len(dna) > 0:
                    yield dna
                    dna = []
            else:
                dna += line[:-1]
        if len(dna) > 0:
            yield dna


    for dna_text in read_dna_text():
        dna = np.array([nuc_to_id[c] for c in dna_text], dtype='int64')
        dna_list.append(dna)

    dna_array = np.empty((len(dna_list), len(dna_list[0])), dtype='int64')

    for i in range(len(dna_list)):
        dna_array[i, :] = dna_list[i]

    (consensus, profile) = solve(dna_array)
    print_consensus_string(consensus)
    print_profile_matrix(profile)
