/*
 Navicat Premium Dump SQL

 Source Server         : 127.0.0.1_3306
 Source Server Type    : MySQL
 Source Server Version : 80035 (8.0.35)
 Source Host           : 127.0.0.1:3306
 Source Schema         : seaorm

 Target Server Type    : MySQL
 Target Server Version : 80035 (8.0.35)
 File Encoding         : 65001

 Date: 19/09/2024 09:10:41
*/

SET NAMES utf8mb4;
SET FOREIGN_KEY_CHECKS = 0;

-- ----------------------------
-- Table structure for wb_comment
-- ----------------------------
DROP TABLE IF EXISTS `wb_comment`;
CREATE TABLE `wb_comment`  (
  `cid` int UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '主键',
  `pid` bigint UNSIGNED NOT NULL COMMENT 'post id',
  `uid` int UNSIGNED NOT NULL COMMENT '用户id',
  `content` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '内容',
  `status` tinyint UNSIGNED NOT NULL COMMENT '状态 0有效 1无效',
  `created_time` int UNSIGNED NOT NULL COMMENT '创建时间',
  `updated_time` int UNSIGNED NOT NULL COMMENT '更新时间',
  PRIMARY KEY (`cid`) USING BTREE,
  INDEX `pid`(`pid` ASC) USING BTREE,
  INDEX `uid`(`uid` ASC) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Table structure for wb_feed_00
-- ----------------------------
DROP TABLE IF EXISTS `wb_feed_00`;
CREATE TABLE `wb_feed_00`  (
  `fid` bigint UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '主键',
  `uid` int UNSIGNED NOT NULL COMMENT '用户id',
  `pid` bigint UNSIGNED NOT NULL COMMENT 'post id',
  `created_time` int UNSIGNED NOT NULL DEFAULT 0 COMMENT '创建时间',
  PRIMARY KEY (`fid`) USING BTREE,
  INDEX `uid_pid`(`uid` ASC, `pid` ASC) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 42 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Table structure for wb_feed_01
-- ----------------------------
DROP TABLE IF EXISTS `wb_feed_01`;
CREATE TABLE `wb_feed_01`  (
  `fid` bigint UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '主键',
  `uid` int UNSIGNED NOT NULL COMMENT '用户id',
  `pid` bigint UNSIGNED NOT NULL COMMENT 'post id',
  `created_time` int UNSIGNED NOT NULL COMMENT '创建时间',
  PRIMARY KEY (`fid`) USING BTREE,
  INDEX `uid_pid`(`uid` ASC, `pid` ASC) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 17 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Table structure for wb_feed_02
-- ----------------------------
DROP TABLE IF EXISTS `wb_feed_02`;
CREATE TABLE `wb_feed_02`  (
  `fid` bigint UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '主键',
  `uid` int UNSIGNED NOT NULL COMMENT '用户id',
  `pid` bigint UNSIGNED NOT NULL COMMENT 'post id',
  `created_time` int UNSIGNED NOT NULL COMMENT '创建时间',
  PRIMARY KEY (`fid`) USING BTREE,
  INDEX `uid_pid`(`uid` ASC, `pid` ASC) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 11 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Table structure for wb_feed_03
-- ----------------------------
DROP TABLE IF EXISTS `wb_feed_03`;
CREATE TABLE `wb_feed_03`  (
  `fid` bigint UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '主键',
  `uid` int UNSIGNED NOT NULL COMMENT '用户id',
  `pid` bigint UNSIGNED NOT NULL COMMENT 'post id',
  `created_time` int UNSIGNED NOT NULL COMMENT '创建时间',
  PRIMARY KEY (`fid`) USING BTREE,
  INDEX `uid_pid`(`uid` ASC, `pid` ASC) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 12 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Table structure for wb_feed_04
-- ----------------------------
DROP TABLE IF EXISTS `wb_feed_04`;
CREATE TABLE `wb_feed_04`  (
  `fid` bigint UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '主键',
  `uid` int UNSIGNED NOT NULL COMMENT '用户id',
  `pid` bigint UNSIGNED NOT NULL COMMENT 'post id',
  `created_time` int UNSIGNED NOT NULL COMMENT '创建时间',
  PRIMARY KEY (`fid`) USING BTREE,
  INDEX `uid_pid`(`uid` ASC, `pid` ASC) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 6 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Table structure for wb_followee
-- ----------------------------
DROP TABLE IF EXISTS `wb_followee`;
CREATE TABLE `wb_followee`  (
  `id` int UNSIGNED NOT NULL AUTO_INCREMENT COMMENT 'id',
  `uid` int UNSIGNED NOT NULL COMMENT '关注者id',
  `followee_id` int UNSIGNED NOT NULL COMMENT '被关注者id',
  `followee_level` tinyint UNSIGNED NULL DEFAULT NULL COMMENT '被关注者级别',
  `status` tinyint UNSIGNED NOT NULL COMMENT '有效状态 0关注  1取关',
  `created_time` int UNSIGNED NOT NULL COMMENT '创建时间',
  `updated_time` int UNSIGNED NOT NULL COMMENT '更新时间',
  PRIMARY KEY (`id`) USING BTREE,
  INDEX `follower_id`(`uid` ASC) USING BTREE,
  INDEX `followee_id`(`followee_id` ASC) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 4 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Table structure for wb_follower
-- ----------------------------
DROP TABLE IF EXISTS `wb_follower`;
CREATE TABLE `wb_follower`  (
  `id` int UNSIGNED NOT NULL AUTO_INCREMENT COMMENT 'id',
  `uid` int UNSIGNED NOT NULL COMMENT '关注者id',
  `follower_id` int UNSIGNED NOT NULL COMMENT '被关注者id',
  `status` tinyint UNSIGNED NOT NULL COMMENT '有效状态 0关注  1取关',
  `created_time` int UNSIGNED NOT NULL COMMENT '创建时间',
  `updated_time` int UNSIGNED NOT NULL COMMENT '更新时间',
  PRIMARY KEY (`id`) USING BTREE,
  INDEX `follower_id`(`uid` ASC) USING BTREE,
  INDEX `followee_id`(`follower_id` ASC) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 4 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Table structure for wb_like
-- ----------------------------
DROP TABLE IF EXISTS `wb_like`;
CREATE TABLE `wb_like`  (
  `lid` int UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '主键',
  `pid` bigint UNSIGNED NOT NULL COMMENT 'post id',
  `uid` int UNSIGNED NOT NULL COMMENT '用户id',
  PRIMARY KEY (`lid`) USING BTREE,
  INDEX `pid`(`pid` ASC) USING BTREE,
  INDEX `uid`(`uid` ASC) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Table structure for wb_post_00
-- ----------------------------
DROP TABLE IF EXISTS `wb_post_00`;
CREATE TABLE `wb_post_00`  (
  `pid` bigint UNSIGNED NOT NULL AUTO_INCREMENT COMMENT 'post id',
  `uid` int UNSIGNED NOT NULL COMMENT '用户id',
  `content` varchar(140) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '微博内容',
  `status` tinyint UNSIGNED NOT NULL COMMENT '有效状态 0有效 1删除',
  `created_time` int UNSIGNED NOT NULL COMMENT '创建时间',
  `updated_time` int UNSIGNED NOT NULL COMMENT '更新时间',
  PRIMARY KEY (`pid`) USING BTREE,
  INDEX `uid`(`uid` ASC) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 143566942450479105 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Table structure for wb_post_01
-- ----------------------------
DROP TABLE IF EXISTS `wb_post_01`;
CREATE TABLE `wb_post_01`  (
  `pid` bigint UNSIGNED NOT NULL AUTO_INCREMENT COMMENT 'post id',
  `uid` int UNSIGNED NOT NULL COMMENT '用户id',
  `content` varchar(140) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '微博内容',
  `status` tinyint UNSIGNED NOT NULL COMMENT '有效状态 0有效 1删除',
  `created_time` int UNSIGNED NOT NULL COMMENT '创建时间',
  `updated_time` int UNSIGNED NOT NULL COMMENT '更新时间',
  PRIMARY KEY (`pid`) USING BTREE,
  INDEX `uid`(`uid` ASC) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 144072249832370177 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Table structure for wb_post_02
-- ----------------------------
DROP TABLE IF EXISTS `wb_post_02`;
CREATE TABLE `wb_post_02`  (
  `pid` bigint UNSIGNED NOT NULL AUTO_INCREMENT COMMENT 'post id',
  `uid` int UNSIGNED NOT NULL COMMENT '用户id',
  `content` varchar(140) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '微博内容',
  `status` tinyint UNSIGNED NOT NULL COMMENT '有效状态 0有效 1删除',
  `created_time` int UNSIGNED NOT NULL COMMENT '创建时间',
  `updated_time` int UNSIGNED NOT NULL COMMENT '更新时间',
  PRIMARY KEY (`pid`) USING BTREE,
  INDEX `uid`(`uid` ASC) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 143566942450479105 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Table structure for wb_post_03
-- ----------------------------
DROP TABLE IF EXISTS `wb_post_03`;
CREATE TABLE `wb_post_03`  (
  `pid` bigint UNSIGNED NOT NULL AUTO_INCREMENT COMMENT 'post id',
  `uid` int UNSIGNED NOT NULL COMMENT '用户id',
  `content` varchar(140) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '微博内容',
  `status` tinyint UNSIGNED NOT NULL COMMENT '有效状态 0有效 1删除',
  `created_time` int UNSIGNED NOT NULL COMMENT '创建时间',
  `updated_time` int UNSIGNED NOT NULL COMMENT '更新时间',
  PRIMARY KEY (`pid`) USING BTREE,
  INDEX `uid`(`uid` ASC) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 144072245000531969 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Table structure for wb_post_04
-- ----------------------------
DROP TABLE IF EXISTS `wb_post_04`;
CREATE TABLE `wb_post_04`  (
  `pid` bigint UNSIGNED NOT NULL AUTO_INCREMENT COMMENT 'post id',
  `uid` int UNSIGNED NOT NULL COMMENT '用户id',
  `content` varchar(140) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '微博内容',
  `status` tinyint UNSIGNED NOT NULL COMMENT '有效状态 0有效 1删除',
  `created_time` int UNSIGNED NOT NULL COMMENT '创建时间',
  `updated_time` int UNSIGNED NOT NULL COMMENT '更新时间',
  PRIMARY KEY (`pid`) USING BTREE,
  INDEX `uid`(`uid` ASC) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 144072870513864705 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Table structure for wb_post_index_00
-- ----------------------------
DROP TABLE IF EXISTS `wb_post_index_00`;
CREATE TABLE `wb_post_index_00`  (
  `pid` bigint UNSIGNED NOT NULL AUTO_INCREMENT COMMENT 'post id',
  `uid` int UNSIGNED NOT NULL COMMENT '用户id',
  `created_time` int UNSIGNED NOT NULL COMMENT '创建时间',
  PRIMARY KEY (`pid`) USING BTREE,
  INDEX `uid`(`uid` ASC) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 143566942450479105 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Table structure for wb_post_index_01
-- ----------------------------
DROP TABLE IF EXISTS `wb_post_index_01`;
CREATE TABLE `wb_post_index_01`  (
  `pid` bigint UNSIGNED NOT NULL AUTO_INCREMENT COMMENT 'post id',
  `uid` int UNSIGNED NOT NULL COMMENT '用户id',
  `created_time` int UNSIGNED NOT NULL COMMENT '创建时间',
  PRIMARY KEY (`pid`) USING BTREE,
  INDEX `uid`(`uid` ASC) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 143566942450479105 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Table structure for wb_post_index_02
-- ----------------------------
DROP TABLE IF EXISTS `wb_post_index_02`;
CREATE TABLE `wb_post_index_02`  (
  `pid` bigint UNSIGNED NOT NULL AUTO_INCREMENT COMMENT 'post id',
  `uid` int UNSIGNED NOT NULL COMMENT '用户id',
  `created_time` int UNSIGNED NOT NULL COMMENT '创建时间',
  PRIMARY KEY (`pid`) USING BTREE,
  INDEX `uid`(`uid` ASC) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 143566942450479105 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Table structure for wb_post_index_03
-- ----------------------------
DROP TABLE IF EXISTS `wb_post_index_03`;
CREATE TABLE `wb_post_index_03`  (
  `pid` bigint UNSIGNED NOT NULL AUTO_INCREMENT COMMENT 'post id',
  `uid` int UNSIGNED NOT NULL COMMENT '用户id',
  `created_time` int UNSIGNED NOT NULL COMMENT '创建时间',
  PRIMARY KEY (`pid`) USING BTREE,
  INDEX `uid`(`uid` ASC) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 143566942450479105 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Table structure for wb_post_index_04
-- ----------------------------
DROP TABLE IF EXISTS `wb_post_index_04`;
CREATE TABLE `wb_post_index_04`  (
  `pid` bigint UNSIGNED NOT NULL AUTO_INCREMENT COMMENT 'post id',
  `uid` int UNSIGNED NOT NULL COMMENT '用户id',
  `created_time` int UNSIGNED NOT NULL COMMENT '创建时间',
  PRIMARY KEY (`pid`) USING BTREE,
  INDEX `uid`(`uid` ASC) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 143566942450479105 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Table structure for wb_super_follower
-- ----------------------------
DROP TABLE IF EXISTS `wb_super_follower`;
CREATE TABLE `wb_super_follower`  (
  `id` int UNSIGNED NOT NULL AUTO_INCREMENT,
  `uid` int UNSIGNED NOT NULL DEFAULT 0 COMMENT '大V',
  `follower_id` int UNSIGNED NOT NULL DEFAULT 0 COMMENT '粉丝',
  PRIMARY KEY (`id`) USING BTREE,
  INDEX `uid`(`uid` ASC) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Table structure for wb_user
-- ----------------------------
DROP TABLE IF EXISTS `wb_user`;
CREATE TABLE `wb_user`  (
  `uid` int UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '用户id',
  `phone` char(11) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '手机号',
  `password` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '密码',
  `created_time` int UNSIGNED NOT NULL COMMENT '创建时间',
  `updated_time` int UNSIGNED NOT NULL COMMENT '更新时间',
  PRIMARY KEY (`uid`) USING BTREE,
  INDEX `phone_password`(`phone` ASC, `password` ASC) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 9 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Table structure for wb_user_info
-- ----------------------------
DROP TABLE IF EXISTS `wb_user_info`;
CREATE TABLE `wb_user_info`  (
  `uid` int UNSIGNED NOT NULL COMMENT '用户id',
  `level` tinyint UNSIGNED NOT NULL DEFAULT 0 COMMENT '级别',
  `is_super` tinyint UNSIGNED NULL DEFAULT 0 COMMENT '是否为超级用户',
  `follow_count` smallint UNSIGNED NOT NULL DEFAULT 0 COMMENT '关注的数量',
  `fans_count` int UNSIGNED NOT NULL DEFAULT 0 COMMENT '粉丝的数量',
  `nickname` varchar(60) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT '昵称',
  `avatar` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT '头像',
  `gender` tinyint UNSIGNED NULL DEFAULT NULL COMMENT '性别 0未知 1男 2女',
  `birthday` int UNSIGNED NULL DEFAULT NULL COMMENT '生日',
  `updated_time` int UNSIGNED NOT NULL COMMENT '更新时间',
  PRIMARY KEY (`uid`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Table structure for wb_user_phone
-- ----------------------------
DROP TABLE IF EXISTS `wb_user_phone`;
CREATE TABLE `wb_user_phone`  (
  `id` int UNSIGNED NOT NULL AUTO_INCREMENT,
  `phone` char(11) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '用户手机号',
  `uid` int UNSIGNED NOT NULL COMMENT '用户uid',
  PRIMARY KEY (`id`) USING BTREE,
  INDEX `phone`(`phone` ASC) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 5 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = Dynamic;

SET FOREIGN_KEY_CHECKS = 1;
