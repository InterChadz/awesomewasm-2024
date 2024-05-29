<template>
    <section class="team">
      <h2>Meet the Team</h2>
      <div class="team-members">
        <div class="team-member secondary border-0 rounded-3 p-3" v-for="member in teamMembers" :key="member.name">
          <img src="@/assets/team.png" :alt="member.name" class="team-photo" />
          <h3>{{ member.name }}</h3>
          <div class="social-links">
            <a :href="`https://x.com/${member.twitter}`" target="_blank">
              <img src="@/assets/twitter-black.svg" alt="Twitter" class="icon" />
            </a>
            <a :href="`https://github.com/${member.twitter}`" target="_blank">
              <img src="@/assets/github-black.svg" alt="GitHub" class="icon" />
            </a>
          </div>
        </div>
      </div>
    </section>
  </template>
  
  <script>
  export default {
    name: 'TeamComponent',
    data() {
      return {
        teamMembers: [
          {
            name: 'Gjermund Garaba',
            image: '@/assets/team.png',
            twitter: 'GjermundGaraba'
          },
          {
            name: 'magiodev',
            image: '@/assets/team.png',
            twitter: 'magiodev'
          },
          {
            name: 'arjanjohan',
            image: '@/assets/team.png',
            twitter: 'arjanjohan'
          }
        ]
      };
    }
  };
  </script>
  
  