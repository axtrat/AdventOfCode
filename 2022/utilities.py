from time import time

class Timer:
    def __init__(self) -> None:
        self.__start = time()
    
    def restart(self):
        self.__start = time()
    
    def __str__(self) -> str:
        seconds = time() - self.__start
        if seconds <= 0:
            return  "--- {} ---".format(str("%.3f" % seconds)+"s")

        notation = ["s", "ms", "µs", "ns"]
        i = 0
        while seconds < 1:
            seconds *= 1000
            i += 1
        return "--- {} ---".format(str("%.3f" % seconds)+notation[i])
    