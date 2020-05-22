import React, {Component} from "react";
import styles from './Header.module.scss';
import {graphql, Link, StaticQuery} from "gatsby";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import { faGithub } from '@fortawesome/free-brands-svg-icons'
import DarkforceImage from "./DarkforceImage";

const headerQuery = graphql`
  query HeaderQuery {
    site {
      siteMetadata {
        repository
        menuLinks {
          name
          link
        }
      }
    }
  }
`;

export interface HeaderProps {
  title: string
}

export default class Header extends Component<HeaderProps> {

  render() {
    const {title} = this.props;

    return (
      <div className={styles.header}>
        <div className={styles.item}>
          <DarkforceImage />
        </div>

      </div>
    )
  }
}
