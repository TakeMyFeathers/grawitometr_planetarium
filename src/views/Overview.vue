<script setup lang="ts">
import useDurationStore from '../stores/duration';
const duration = useDurationStore();

const durationInSeconds = duration.value / 1000

const gravitational_acceleration = [
  {
    name: "Merkury",
    acc: 3.7
  },
  {
    name: "Wenus",
    acc: 8.9
  },
  {
    name: "Ziemia",
    acc: 9.81
  },
  {
    name: "Mars",
    acc: 3.7
  },
  {
    name: "Jowisz",
    acc: 23.1
  },
  {
    name: "Saturn",
    acc: 9
  },
  {
    name: "Uran",
    acc: 8.7
  },
  {
    name: "Neptun",
    acc: 11
  }
]
</script>

<template>
  <div class="flex flex-col text-center ">
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
          <td>{{ planet.acc }}</td>
          <td>{{ planet.acc * durationInSeconds}}</td>
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
      color: #FD5C1E;
    }
  }
}
</style>