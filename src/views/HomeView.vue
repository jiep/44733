<script setup lang="ts">
  import { reactive, ref } from 'vue'

  import LocationList from '@/components/LocationList.vue'
  import { Location }from '@/model/Location'

  const number = ref(44733)

  const locations = reactive({items: new Array<Location>});

  const response = await fetch(`.netlify/functions/number?number=${number.value}`)
  // const data = await response.json()

  const data = {
  "number": 44733,
  "locations": [
    {
      "name": "",
      "address": "CARRETERA SANTA MARTA, 71",
      "city": "ALMENDRALEJO",
      "province": "BADAJOZ",
      "phone": "924660780",
      "series": [
        121,
        122,
        123,
        124,
        125,
        126,
        127,
        128,
        129,
        130,
        131,
        132,
        133,
        134,
        135,
        136,
        137,
        138,
        139,
        140,
        141,
        142,
        143,
        144,
        145,
        146,
        147,
        148,
        149,
        150,
        151,
        152,
        153,
        154,
        155,
        156,
        157,
        158,
        159,
        160,
        161,
        162,
        163,
        164,
        165,
        166,
        167,
        168,
        169,
        170,
        171,
        172,
        173,
        174,
        175,
        176,
        177,
        178,
        179,
        180
      ]
    },
    {
      "name": "",
      "address": "OBISPO, 1",
      "city": "JACA",
      "province": "HUESCA",
      "phone": "974363705",
      "series": [
        1,
        2,
        3,
        4,
        5,
        6,
        7,
        8,
        9,
        10,
        11,
        12,
        13,
        14,
        15,
        16,
        17,
        18,
        19,
        20,
        21,
        22,
        23,
        24,
        25,
        26,
        27,
        28,
        29,
        30,
        31,
        32,
        33,
        34,
        35,
        36,
        37,
        38,
        39,
        40,
        41,
        42,
        43,
        44,
        45,
        46,
        47,
        48,
        49,
        50,
        51,
        52,
        53,
        54,
        55,
        56,
        57,
        58,
        59,
        60
      ]
    },
    {
      "name": "",
      "address": "AVDA. DE LA LIBERTAD, 19-NAVATEJERA",
      "city": "VILLAQUILAMBRE",
      "province": "LEON",
      "phone": "987285512",
      "series": [
        185
      ]
    },
    {
      "name": "",
      "address": "MARMOLES, 41",
      "city": "MÁLAGA",
      "province": "MALAGA",
      "phone": "952307886",
      "series": [
        61,
        62,
        63,
        64,
        65,
        66,
        67,
        68,
        69,
        70,
        71,
        72,
        73,
        74,
        75,
        76,
        77,
        78,
        79,
        80,
        81,
        82,
        83,
        84,
        85,
        86,
        87,
        88,
        89,
        90,
        91,
        92,
        93,
        94,
        95,
        96,
        97,
        98,
        99,
        100,
        101,
        102,
        103,
        104,
        105,
        106,
        107,
        108,
        109,
        110,
        111,
        112,
        113,
        114,
        115,
        116,
        117,
        118,
        119,
        120
      ]
    }
  ]
}

  let items: Array<Location> = data.locations;
  items = items.map(x => new Location(x.name, x.address, x.city, x.province, x.series))
               .sort((a: Location, b:Location) => b.series.length - a.series.length)

  locations.items = items;

  async function onClick(number: any) {
    console.log(number);
    
    const response = await fetch(`.netlify/functions/number?number=${number.toString().padStart(5, '0')}`)
    const data = await response.json()

    const d: Array<Location> = data.locations;

    items = d.map(x => new Location(x.name, x.address, x.city, x.province, x.series))
             .sort((a: Location, b:Location) => b.series.length - a.series.length)

    locations.items = items;

    console.log(locations)
  }

</script>

<template>
    <!-- <Tabs /> -->
    <div class="flex m-4 pt-0 items-center">
      <input type="number" min="0" max="100000"
        placeholder="Introduce un número para buscar" 
        class="px-3 py-3 placeholder-slate-400 text-slate-600 relative bg-white bg-white text-sm border-0 shadow outline-none focus:outline-none focus:ring w-full" 
        v-model="number" 
      />
      <button class="bg-pink-500 text-white active:bg-pink-600 font-bold uppercase text-sm px-6 py-3 shadow hover:shadow-lg outline-none focus:outline-none mr-1 mb-1 ease-linear transition-all duration-150" type="button" @click="onClick(number)"
      >Buscar</button>
    </div>
    <LocationList :items="items" />
</template>
