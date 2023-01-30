from os import path, walk
from pathlib import Path
import redis
import json
import re

superscript_regex = re.compile(r".*\(\d+\)")

root = Path(path.dirname(__file__)).parent

r = redis.Redis(host="127.0.0.1", port=6379, password="foobared", username="default")


def get_kbbi():
    dp_fn = []
    for dp, _, fn in walk(root.joinpath("kbbi")):
        if fn[0].endswith(".json"):
            dp_fn.append((dp, fn))

    return dp_fn


def filepath_gen(path_tuple_list):
    for dp, fn in path_tuple_list:
        for f in fn:
            yield dp, f


def get_kata_tidak_baku_list(path_tuple_list):
    tidak_baku_list = [
        "ajung",
        "akhli",
        "bangkah",
        "bongkol",
        "bonjol",
        "cakmar",
        "cemek",
        "katek",
        "kerkap",
        "langkara",
        "letur",
        "merkah",
        "pekir",
        "rasia",
        "rebet",
        "sahdu",
        "seniwan",
        "seturi",
        "silah",
        "tembel",
        "tombong",
        "unggit",
        "ijin",
    ]
    tidak_baku_list = set(tidak_baku_list)

    for dp, f in filepath_gen(path_tuple_list):
        file = f"{dp}/{f}"
        obj = json.load(open(file))

        for entri in obj["entri"]:
            for ga_baku in entri["bentuk_tidak_baku"]:
                if superscript_regex.match(ga_baku) is None:
                    tidak_baku_list.add(ga_baku.lower())

    return tidak_baku_list


def set_kbbi(path_tuple_list, kata_gak_baku_list):
    # must manually enter in redis-cli
    # FT.CREATE kbbi ON JSON PREFIX 1 bhs: SCHEMA entry TEXT WEIGHT 3.0 pranala TEXT
    res = r.execute_command(
        "FT.CREATE kbbi ON JSON PREFIX 1 bhs: SCHEMA $.nama AS nama TEXT WEIGHT 3.0 $.pranala AS pranala TEXT $.key AS key TAG"
    )
    print("FT.CREATE result:", res)
    i = 0
    lis_len = 109_345
    for dp, f in filepath_gen(path_tuple_list):
        file = f"{dp}/{f}"
        key = f"bhs:{i}"

        if kata_gak_baku_list.__contains__(f[:-5]):
            continue

        r.json().set_file(key, "$", file)
        r.json().set(key, "$.nama", f[:-5])
        r.json().set(key, "$.key", f[:-5])

        if i % 5000 == 0:
            print(f"\rprogress: {str(i/lis_len*100)[:5]}%", end="")
        i += 1

    print(f"\rprogress: 100.0%")
    print(f"kbbi dict set successful {i} uploaded")

    r.ft().dict_add("tidak_baku", *kata_gak_baku_list)


def set_en_words():
    the_dir = root.joinpath("english-words")
    en_word_list = open(the_dir.joinpath("words.txt")).read().split("\n")
    r.ft().dict_add("english", *en_word_list)
    print("english words has been added")


def main():
    kbbi_vec = get_kbbi()
    list_kata_ga_baku = get_kata_tidak_baku_list(kbbi_vec)
    set_kbbi(kbbi_vec, list_kata_ga_baku)
    set_en_words()


if __name__ == "__main__":
    main()
