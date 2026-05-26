alias mrproper := clean
just := just_executable()

target := 'riscv64'
debug := '1'
cpus := '8'
mem := '512'

clean:
    {{ just }} kernel/clean

build:
    {{ just }} target={{target}} debug={{debug}} kernel/build

run:
    {{ just }} target={{target}} debug={{debug}} cpus={{cpus}} mem={{mem}} kernel/run
