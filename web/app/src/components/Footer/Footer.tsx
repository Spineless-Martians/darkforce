import React, {Component} from "react";
import Container from "../Container";
import styles from './Footer.module.scss';
import {Link} from "gatsby";


export default class Footer extends Component {

  render() {
    return (
      <div className={styles.footer}>
        <Container>
          <div className={styles.container}>
            <div className={styles.copyrightRow}>
              <span>Made with  ❤️ by Spineless-Martians</span>
            </div>
          </div>
        </Container>
      </div>
    )
  }
}
