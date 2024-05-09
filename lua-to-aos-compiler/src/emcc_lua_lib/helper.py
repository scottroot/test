import os
import subprocess

IS_DEBUG = os.environ.get('DEBUG', '')


def encode_hex_literals(source):
    return ", ".join([f"0x{x:02x}" for x in source.encode("utf-8")])


def get_extension(file):
    return os.path.splitext(os.path.basename(file))[1][1:]


def is_lua_source_file(file):
    ext = get_extension(file)
    return ext in ["lua", "luac"]


def is_binary_library(file):
    ext = get_extension(file)
    return ext in ["o", "a", "so", "dylib"]


def shell_exec(*cmd_args):
    commands = list(cmd_args)
    try:
        proc = subprocess.run(commands, stdout=subprocess.PIPE, check=True)
        return proc.stdout.decode("utf-8").strip("\n"), proc.returncode
    except subprocess.CalledProcessError as e:
        print(f"Failed running shell_exec: {e}")
        return None, e.returncode


# def debug_print(message, sep=" "):
#     if IS_DEBUG:
#         print(message, sep=sep)
def debug_print(*args, **kwargs):
    if IS_DEBUG:
        print(*args)
