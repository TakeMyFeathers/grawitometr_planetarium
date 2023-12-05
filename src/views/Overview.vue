<script setup lang="ts">
import router from "../router";
import useDurationStore from "../stores/duration";
const duration = useDurationStore();

const durationInSeconds = Math.abs(duration.value) / 1000;

let timer: number;

/*
  acc: przyśpieszenie planety
  gVar: wartość względem przyśpieszenia ziemskiego

*/

const gravitational_acceleration = [
  {
    name: "Merkury",
    acc: 3.7,
    gVar: 0.38,
  },
  {
    name: "Wenus",
    acc: 8.9,
    gVar: 0.9,
  },
  {
    name: "Ziemia",
    acc: 9.81,
    gVar: 1,
  },
  {
    name: "Mars",
    acc: 3.7,
    gVar: 0.38,
  },
  {
    name: "Jowisz",
    acc: 23.1,
    gVar: 2.35,
  },
  {
    name: "Saturn",
    acc: 9,
    gVar: 0.92,
  },
  {
    name: "Uran",
    acc: 8.7,
    gVar: 0.89,
  },
  {
    name: "Neptun",
    acc: 11,
    gVar: 1.12,
  },
];

//Po 20 sekundach wejdz znów na stronę startową
timer = setTimeout(() => {
  router.push({ path: "/" });
}, 20000);
console.log(timer);
</script>

<template>
  <div class="flex flex-col text-center">
    <table class="table">
      <thead>
        <tr>
          <td>Planeta</td>
          <td>Przyśpieszenie grawitacyjne</td>
          <td>Wysokość skoku</td>
        </tr>
      </thead>
      <tbody>
        <tr class="py-5" v-for="planet in gravitational_acceleration">
          <td>{{ planet.name }}</td>
          <td>{{ planet.acc }}m/s²</td>
          <td>
            {{
              (
                (gravitational_acceleration[2].acc * durationInSeconds ** 2) /
                2 /
                planet.gVar
              ).toFixed(2)
            }}m
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<style scoped lang="scss">
.table {
  td {
    border-right: 8px solid white;
    text-align: center;

    &:last-of-type {
      border: none;
    }
  }

  tbody {
    td {
      font-size: 60px;
    }
  }

  thead {
    td {
      width: 33%;
      text-transform: uppercase;
      font-size: 70px;
      color: #fd5c1e;
    }
  }
}
</style>
