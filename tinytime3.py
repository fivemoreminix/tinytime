import sys, os, timeit

if __name__ == '__main__':
    if len(sys.argv) > 1:
        command = ' '.join(sys.argv[1:])
        print('{0}s'.format(timeit.timeit('os.system(command)', number=1, globals=globals())))
    else:
        print('INFO:\n{0} times the given shell command (COMMAND). The number returned is the seconds the process took to execute.\nhttps://github.com/asmoaesl/tinytime/\n'.format(sys.argv[0]))
        print('USAGE:\tpython {0} <COMMAND>'.format(sys.argv[0]))