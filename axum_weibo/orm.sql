-- --------------------------------------------------------
-- 主机:                           127.0.0.1
-- 服务器版本:                        8.0.35 - MySQL Community Server - GPL
-- 服务器操作系统:                      Linux
-- HeidiSQL 版本:                  12.6.0.6765
-- --------------------------------------------------------

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET NAMES utf8 */;
/*!50503 SET NAMES utf8mb4 */;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;

-- 导出  表 seaorm.wb_comment 结构
DROP TABLE IF EXISTS `wb_comment`;
CREATE TABLE IF NOT EXISTS `wb_comment` (
  `cid` int unsigned NOT NULL AUTO_INCREMENT COMMENT '主键',
  `pid` int unsigned NOT NULL COMMENT 'post id',
  `uid` int unsigned NOT NULL COMMENT '用户id',
  `content` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '内容',
  `status` tinyint unsigned NOT NULL COMMENT '状态 0有效 1无效',
  `created_time` int unsigned NOT NULL COMMENT '创建时间',
  `updated_time` int unsigned NOT NULL COMMENT '更新时间',
  PRIMARY KEY (`cid`),
  KEY `pid` (`pid`) USING BTREE,
  KEY `uid` (`uid`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- 数据导出被取消选择。

-- 导出  表 seaorm.wb_feed 结构
DROP TABLE IF EXISTS `wb_feed`;
CREATE TABLE IF NOT EXISTS `wb_feed` (
  `fid` int unsigned NOT NULL AUTO_INCREMENT COMMENT '主键',
  `pid` int unsigned NOT NULL COMMENT 'post id',
  `uid` int unsigned NOT NULL COMMENT '用户id',
  PRIMARY KEY (`fid`) USING BTREE,
  KEY `uid_pid` (`uid`,`pid`) USING BTREE
) ENGINE=InnoDB AUTO_INCREMENT=6 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- 数据导出被取消选择。

-- 导出  表 seaorm.wb_follower 结构
DROP TABLE IF EXISTS `wb_follower`;
CREATE TABLE IF NOT EXISTS `wb_follower` (
  `id` int unsigned NOT NULL AUTO_INCREMENT COMMENT 'id',
  `follower_id` int unsigned NOT NULL COMMENT '关注者id',
  `followee_id` int unsigned NOT NULL COMMENT '被关注者id',
  `status` tinyint unsigned NOT NULL COMMENT '有效状态 0关注  1取关',
  `created_time` int unsigned NOT NULL COMMENT '创建时间',
  `updated_time` int unsigned NOT NULL COMMENT '更新时间',
  PRIMARY KEY (`id`),
  KEY `follower_id` (`follower_id`) USING BTREE,
  KEY `followee_id` (`followee_id`) USING BTREE
) ENGINE=InnoDB AUTO_INCREMENT=3 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- 数据导出被取消选择。

-- 导出  表 seaorm.wb_like 结构
DROP TABLE IF EXISTS `wb_like`;
CREATE TABLE IF NOT EXISTS `wb_like` (
  `lid` int unsigned NOT NULL AUTO_INCREMENT COMMENT '主键',
  `pid` int unsigned NOT NULL COMMENT 'post id',
  `uid` int unsigned NOT NULL COMMENT '用户id',
  PRIMARY KEY (`lid`) USING BTREE,
  KEY `pid` (`pid`) USING BTREE,
  KEY `uid` (`uid`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- 数据导出被取消选择。

-- 导出  表 seaorm.wb_post 结构
DROP TABLE IF EXISTS `wb_post`;
CREATE TABLE IF NOT EXISTS `wb_post` (
  `pid` int unsigned NOT NULL AUTO_INCREMENT COMMENT 'post id',
  `uid` int unsigned NOT NULL COMMENT '用户id',
  `content` varchar(140) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '微博内容',
  `status` tinyint unsigned NOT NULL COMMENT '有效状态 0有效 1删除',
  `created_time` int unsigned NOT NULL COMMENT '创建时间',
  `updated_time` int unsigned NOT NULL COMMENT '更新时间',
  PRIMARY KEY (`pid`),
  KEY `uid` (`uid`) USING BTREE
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- 数据导出被取消选择。

-- 导出  表 seaorm.wb_user 结构
DROP TABLE IF EXISTS `wb_user`;
CREATE TABLE IF NOT EXISTS `wb_user` (
  `uid` int unsigned NOT NULL AUTO_INCREMENT COMMENT '用户id',
  `phone` char(11) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '手机号',
  `password` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '密码',
  `created_time` int unsigned NOT NULL COMMENT '创建时间',
  `updated_time` int unsigned NOT NULL COMMENT '更新时间',
  PRIMARY KEY (`uid`),
  KEY `phone_password` (`phone`,`password`) USING BTREE
) ENGINE=InnoDB AUTO_INCREMENT=3 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- 数据导出被取消选择。

-- 导出  表 seaorm.wb_user_info 结构
DROP TABLE IF EXISTS `wb_user_info`;
CREATE TABLE IF NOT EXISTS `wb_user_info` (
  `uid` int unsigned NOT NULL COMMENT '用户id',
  `nickname` varchar(60) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '昵称',
  `avatar` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '头像',
  `gender` tinyint unsigned DEFAULT NULL COMMENT '性别 0未知 1男 2女',
  `birthday` int unsigned DEFAULT NULL COMMENT '生日',
  `updated_time` int unsigned NOT NULL COMMENT '更新时间',
  PRIMARY KEY (`uid`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- 数据导出被取消选择。

/*!40103 SET TIME_ZONE=IFNULL(@OLD_TIME_ZONE, 'system') */;
/*!40101 SET SQL_MODE=IFNULL(@OLD_SQL_MODE, '') */;
/*!40014 SET FOREIGN_KEY_CHECKS=IFNULL(@OLD_FOREIGN_KEY_CHECKS, 1) */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40111 SET SQL_NOTES=IFNULL(@OLD_SQL_NOTES, 1) */;
