import sys
from ctypes import *
import time
import gc
def lock_keyboards():
    windll.user32.BlockInput(True)
    time.sleep(5)
    windll.user32.BlockInput(False)
