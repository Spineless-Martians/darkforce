
module.exports = {
  siteMetadata: {
    title: `Darkforce`,
    description: `Darkforce`,
    author: `Joatin Granlund <granlundjoatin@icloud.com>, lykata`,
    repository: `https://github.com/Spineless-Martians/darkforce`,
    menuItems:[
      {
        name:'Home',
        link:'/',
        icon: 'home'
      },
      {
        name:'Dag List',
        link:'/dags',
        icon: 'book'
      },
      {
        name:'Users',
        link:'/users',
        icon: 'rss'
      },
      {
        name:'Settings',
        link:'/settings',
        icon: 'rss'
      }
    ]
  },
  plugins: [
    `gatsby-plugin-react-helmet`,
    {
      resolve: `gatsby-source-filesystem`,
      options: {
        name: `images`,
        path: `${__dirname}/src/images`,
      },
    },
    `gatsby-transformer-sharp`,
    `gatsby-plugin-sharp`,
    `gatsby-plugin-typescript`,
    `gatsby-plugin-sass`,
    {
      resolve: `gatsby-plugin-manifest`,
      options: {
        name: `darkforce`,
        short_name: `darkforce`,
        start_url: `/`,
        background_color: `#292b33`,
        theme_color: `#292b33`,
        display: `minimal-ui`,
        icon: `src/images/gatsby-icon.png`, // This path is relative to the root of the site.
      },
    },
    `gatsby-plugin-offline`
  ],
  proxy: [
    {
      prefix: "/graphql",
      url: "http://localhost:3000/graphql/",
    },
    {
      prefix: "/playground",
      url: "http://localhost:3000/playground/",
    }
  ],
};
