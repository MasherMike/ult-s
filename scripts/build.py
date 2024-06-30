import os
import shutil
import sys
from zipfile import ZipFile
import subprocess

try:
    inputs = ("".join(sys.argv)).lower()
    inputs = inputs.replace('build.py', "")
    inputs = inputs.replace('\n', "")
    print(inputs)
except IndexError:
    raise Exception("No version inputted!")

inputs = inputs.replace('build.py ', "")

def copytree(src, dst, symlinks=False, ignore=None):
    for item in os.listdir(src):
        s = os.path.join(src, item)
        d = os.path.join(dst, item)
        if os.path.isdir(s):
            shutil.copytree(s, d, symlinks, ignore)
        else:
            shutil.copy2(s, d)

subprocess.run('cargo skyline build --release --features="main_nro"', shell=True)
print(os.getcwd())
old = r"target\aarch64-skyline-switch\release\libplugin.nro"
new = r"releases/ultimate/mods/Ultimate S Arcropolis"
old_rename = r"libplugin.nro"
rename = r"plugin.nro"

def empty_folder():
    folder = r'releases'
    for filename in os.listdir(folder):
        file_path = os.path.join(folder, filename)
        try:
            if os.path.isfile(file_path) or os.path.islink(file_path):
                os.unlink(file_path)
            elif os.path.isdir(file_path):
                shutil.rmtree(file_path)
        except Exception as e:
            print('Failed to delete %s. Reason: %s' % (file_path, e))

def get_all_file_paths(directory):
  
    # initializing empty file paths list
    file_paths = []
  
    # crawling through directory and subdirectories
    for root, directories, files in os.walk(directory):
        for filename in files:
            # join the two strings in order to form the full filepath.
            filepath = os.path.join(root, filename)
            file_paths.append(filepath)
  
    # returning all file paths
    return file_paths       

print("Finished Building... now compiling Romfs")
print(os.chdir(".."))
if os.path.exists(r'target'):
    os.chdir(r'target')
    print(os.listdir())
    if os.path.exists(r'aarch64-skyline-switch'):
        os.chdir(r'aarch64-skyline-switch')
        print(os.listdir())
        if os.path.exists(r'release'):
            os.chdir(r'release')
            #print(os.listdir())
            old = os.path.join(os.path.abspath(os.getcwd()), r'libplugin.nro')
            os.chdir('../')
            os.chdir('../')
            os.chdir('../')
            print(os.getcwd())
            if os.path.exists(r'releases'):
                empty_folder()
            if os.path.exists(new) == False:
                os.makedirs(new)
            #print(old)
            shutil.move(old, new)
            #print(os.listdir())
            if os.path.exists(r'releases/ultimate/mods/Ultimate S Arcropolis') == False:
                os.makedirs(r'releases/ultimate/mods/Ultimate S Arcropolis')
            shutil.move(os.path.join(new, r'libplugin.nro'), os.path.join(new, r'plugin.nro'))
            if os.path.exists(r'romfs'):
                print("Starting copy")
                copytree(r'romfs', r'releases/ultimate/mods/Ultimate S Arcropolis')
                print("Copying from romfs finished, now zipping")
            else:
                print("Error! No romfs folder! Please check your install")

            #Version Text
            f = open(r'releases/ultimate/mods/Ultimate S Arcropolis/version.txt',"w")
            f.write(f"v.{inputs}")
            f.close()
            shutil.copy(r'readme.txt', r'releases/readme.txt')
            shutil.copy(r'Ultimate S Setup Tool.exe', r'releases/Ultimate S Setup Tool.exe')
            shutil.copy(r'Ultimate S Setup Tool.py', r'releases/Ultimate S Setup Tool.py')
            shutil.copytree(r'resources', r'releases/resources')

            if os.path.exists(r'releases/Ultimate S Arcropolis.zip'):
                os.remove(r'releases/Ultimate S Arcropolis.zip')
            file_paths = get_all_file_paths(new)
            with ZipFile(r'releases/Ultimate S Arcropolis.zip','w') as zip:
                for file in file_paths:
                    zip.write(file)
            print("Done!")
        else:
            print('aarch64-skyline-switch does not exist')
    else:
        print('aarch64-skyline-switch does not exist')
else:
    print('target does not exist')
