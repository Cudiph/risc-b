import string
import requests

ENDPOINT = "http://localhost:8080/api/v1/cek"

def test_format():
    pload = {
        "query": "ini adalah sdikit   kesalhan\t tapi gak ada yang peduli",
        "format": "HTML",
        "tolerance": "LOW",
        "correction": False,
        "english": False,
        "tidak_baku": False,
        "result_vec": False,
    }

    # 1
    a = requests.post(ENDPOINT, json=pload).json()
    assert(a['result'] == "ini adalah <b>sdikit</b>   <b>kesalhan</b>\t <b>tapi</b> <b>gak</b> ada yang peduli")

    # 2
    pload["format"] = "MD"
    a = requests.post(ENDPOINT, json=pload).json()
    assert(a['result'] == "ini adalah **sdikit**   **kesalhan**\t **tapi** **gak** ada yang peduli")

    

def test_correction():
    pload = {
        "query": "ini sepertrnya ada seidkit kekeliuran",
        "format": "MD",
        "tolerance": "LOW",
        "correction": True,
        "english": False,
        "tidak_baku": False,
        "result_vec": False,
    }

    # 1
    a = requests.post(ENDPOINT, json=pload).json()
    assert(a['result'] == "ini **sepertinya** ada **seidkit** **kekeliuran**")

    # 2
    pload["tolerance"] = "MEDIUM"
    a = requests.post(ENDPOINT, json=pload).json()
    assert(a['result'] == "ini **sepertinya** ada **sedikit** **kekeliruan**")



def test_english():
    pload = {
        "query": "it has english words & ini adalah bahasa Indonesia",
        "format": "MD",
        "tolerance": "LOW",
        "correction": False,
        "english": True,
        "tidak_baku": False,
        "result_vec": False,
    }

    # 1
    a = requests.post(ENDPOINT, json=pload).json()
    assert(a['valid'] == True)

    # 2
    pload["english"] = False
    a = requests.post(ENDPOINT, json=pload).json()
    assert(a['result'] == "**it** has **english** **words** & ini adalah bahasa Indonesia")


def test_tidak_baku():
    pload = {
        "query": "apotik tapi ijin ustadz aktip karir lembap ijasah",
        "format": "MD",
        "tolerance": "LOW",
        "correction": False,
        "english": True,
        "tidak_baku": True,
        "result_vec": False,
    }

    a = requests.post(ENDPOINT, json=pload).json()
    assert(a['valid'] == True)

def test_result_vec():
    pload = {
        "query": "ini adalah contoh taip ga ada yang pduli",
        "format": "MD",
        "tolerance": "LOW",
        "correction": False,
        "english": True,
        "tidak_baku": True,
        "result_vec": False,
    }

    a = requests.post(ENDPOINT, json=pload).json()
    pload["result_vec"] = True
    b = requests.post(ENDPOINT, json=pload).json()
    assert(a["result"] == ''.join(b["result_vec"]))


def test_capital_case():
    invalid = [
        "iNI",
        "inI",
        "TiDaK",
    ]
    pload = {
        "query": f"{invalid[0]} valid, {invalid[1]} {invalid[2]} Valid VALID",
        "format": "MD",
        "tolerance": "LOW",
        "correction": False,
        "english": False,
        "tidak_baku": False,
        "result_vec": False,
    }

    a = requests.post(ENDPOINT, json=pload).json()

    for i in range(len(a['reccomendation'])):
        assert(a['reccomendation'][i][1] == invalid[i])

def test_ascii():
    pload = {
        "query": string.printable,
        "format": "MD",
        "tolerance": "LOW",
        "correction": False,
        "english": False,
        "tidak_baku": False,
        "result_vec": False,
    }

    a = requests.post(ENDPOINT, json=pload)
    assert(a.ok)



if __name__ == "__main__":
    test_format()
    test_correction()
    test_english()
    test_tidak_baku()
    test_result_vec()
    test_capital_case()
    test_ascii()
    print("PASSED")

